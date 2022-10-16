use crate::errors::*;
use std::process::Command;

pub fn run() -> Result<()> {
    info!("Executing updpkgsums");
    let status = Command::new("updpkgsums").spawn()?.wait()?;
    if !status.success() {
        bail!("Process exited with error code: {:?}", status);
    }
    info!("Successfully updated checksums in PKGBUILD");
    Ok(())
}
