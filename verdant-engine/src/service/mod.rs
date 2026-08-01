#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::{run_service, install_service, uninstall_service};

#[cfg(not(windows))]
pub fn run_service() -> anyhow::Result<()> {
    anyhow::bail!("Windows service is only supported on Windows");
}

#[cfg(not(windows))]
pub fn install_service() -> anyhow::Result<()> {
    anyhow::bail!("Windows service is only supported on Windows");
}

#[cfg(not(windows))]
pub fn uninstall_service() -> anyhow::Result<()> {
    anyhow::bail!("Windows service is only supported on Windows");
}
