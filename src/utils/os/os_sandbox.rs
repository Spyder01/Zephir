use std::io;
use std::path::Path;
use rlimit::{setrlimit, Resource};
use nix::unistd::{chroot, setgid, setuid, Gid, Uid};

pub fn apply_unix_sandbox(is_root: bool, chroot_dir: Option<&Path>, cpu_time: u64, memory_limit: u64, size_limit: u64) -> io::Result<()> {
    // CPU time
    let _ = setrlimit(Resource::CPU, cpu_time, cpu_time);
    // Max address space (memory)
    let _ = setrlimit(Resource::AS, memory_limit, memory_limit);
    // Max file size
    let _ = setrlimit(Resource::FSIZE, size_limit, size_limit);

    if is_root {
        if let Some(dir) = chroot_dir {
            // Chroot into the sandbox directory
            chroot(dir).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("chroot failed: {e}")))?;
            // Change to "/" inside the chroot
            std::env::set_current_dir("/")?;
        }

        // Drop privileges: switch to nobody/nogroup
        let _ = setgid(Gid::from_raw(65534)); // nogroup
        let _ = setuid(Uid::from_raw(65534)); // nobody
    }

    Ok(())
}
