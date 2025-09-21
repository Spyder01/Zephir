use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf, Component};
use tar::{Builder, Archive};
use zstd::stream::{Encoder, Decoder};
use walkdir::WalkDir;

pub fn compress_dir_to_zstd(src_dir: &str, dst_file: &str, level: i32) -> std::io::Result<()> {
    let file = File::create(dst_file)?;
    let buf = BufWriter::new(file);

    let encoder = Encoder::new(buf, level)?;
    let mut encoder = encoder.auto_finish();

    let mut tar_builder = Builder::new(&mut encoder);

    for entry in WalkDir::new(src_dir) {
        let entry = entry?;
        let path = entry.path();

        if path == std::path::Path::new(src_dir) {
            continue;
        }

        let relative_path = path.strip_prefix(src_dir).unwrap();

        if path.is_file() {
            tar_builder.append_path_with_name(path, relative_path)?;
        } else if path.is_dir() {
            if relative_path.as_os_str().len() > 0 {
                tar_builder.append_dir(relative_path, path)?;
            }
        }
    }

    tar_builder.finish()?;
    Ok(())
}


fn sanitize_entry_path(entry_path: &Path, dest: &Path) -> io::Result<PathBuf> {
    if entry_path.is_absolute() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Absolute paths not allowed"));
    }

    let mut safe = PathBuf::from(dest);
    for comp in entry_path.components() {
        match comp {
            Component::Normal(name) => safe.push(name),
            Component::CurDir => { /* skip */ }
            Component::ParentDir => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Parent components not allowed"));
            }
            _ => {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid path component"));
            }
        }
    }

    if !safe.starts_with(dest) {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Path escapes sandbox"));
    }

    Ok(safe)
}

pub fn decompress_zstd_to_dir(src_file: &str, dst_dir: &str) -> io::Result<()> {
    let dst_dir_path = Path::new(dst_dir);

    fs::create_dir_all(&dst_dir_path)?;

    let file = File::open(src_file)?;
    let buf = BufReader::new(file);
    let decoder = Decoder::new(buf)?;
    let mut archive = Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?;
        let safe_path = sanitize_entry_path(&entry_path, &dst_dir_path)?;

        let header = entry.header().clone();

        if header.entry_type().is_dir() {
            fs::create_dir_all(&safe_path)?;
            continue;
        }

        if header.entry_type().is_symlink() {
            return Err(io::Error::new(io::ErrorKind::Other, "Symlinks not allowed in archive"));
        }

        if let Some(parent) = safe_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let mut out = File::create(&safe_path)?;
        io::copy(&mut entry, &mut out)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(mode) = header.mode() {
                let perm = fs::Permissions::from_mode(mode as u32);
                out.set_permissions(perm)?;
            }
        }
    }

    Ok(())
}
