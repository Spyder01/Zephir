pub fn has_root_privilege() -> bool {
    #[cfg(unix)]
    {
        nix::unistd::Uid::current().is_root()
    }
    #[cfg(windows)]
    {
        is_elevated::is_elevated().unwrap_or(false)
    }
}
