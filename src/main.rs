mod args;
mod errors;
mod makepkg;
mod updpkgsums;

use crate::errors::*;
use clap::Parser;
use env_logger::Env;
use std::env;
use std::fs;
use std::process::Command;
use tempdir::TempDir;

fn update() -> Result<()> {
    let tmp_dir = TempDir::new("updlockfiles")?;
    let tmp_path = tmp_dir.path();
    debug!("Generated temporary directory: {:?}", tmp_path);

    info!("Generating new lockfiles...");
    let cmd = "source ./PKGBUILD && cd src && updlockfiles";
    debug!("Executing shell command: {:?}", cmd);
    let status = Command::new("sh")
        .args(&["-ec", cmd])
        .env("outdir", tmp_path)
        .spawn()?
        .wait()?;
    if !status.success() {
        bail!("Process exited with error code: {:?}", status);
    }

    info!("Finished generating lockfiles, collecting results...");
    for entry in fs::read_dir(tmp_path)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().context("File in directory has no name")?;
        info!("Copy {:?} => {:?}", path, filename);
        fs::copy(&path, filename)
            .with_context(|| anyhow!("Failed to copy file {:?} to current directory", path))?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let args = args::Args::parse();

    let log_level = match args.verbose {
        0 => "updlockfiles=info",
        1 => "info,updlockfiles=debug",
        2 => "debug",
        _ => "trace",
    };
    env_logger::init_from_env(Env::default().default_filter_or(log_level));

    if let Some(path) = &args.path {
        env::set_current_dir(path)
            .with_context(|| anyhow!("Failed to change into directory: {:?}", path))?;
    }

    makepkg::unpack().context("Failed to run makepkg")?;
    update().context("Failed to update lockfiles")?;
    updpkgsums::run().context("Failed to run updpkgsums")?;

    Ok(())
}
