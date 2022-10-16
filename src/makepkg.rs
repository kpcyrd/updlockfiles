use crate::errors::*;
use std::process::Command;

pub fn unpack() -> Result<()> {
    info!("Executing makepkg to unpack source code...");
    let status = Command::new("makepkg")
        .args(&["--nodeps", "--noprepare", "--nobuild"])
        .spawn()?
        .wait()?;
    if !status.success() {
        bail!("Process exited with error code: {:?}", status);
    }
    info!("Finished unpacking source code");
    Ok(())
}
