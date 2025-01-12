use anyhow::Context;
use std::fs;

pub fn init() -> anyhow::Result<()> {
    fs::create_dir(".git").context("Could not create .git directory")?;
    fs::create_dir(".git/objects").context("Could not create .git/objects directory")?;
    fs::create_dir(".git/refs").context("Could not create .git/refs directory")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n").context("Could not write to .git/HEAD")?;
    println!("Initialized git directory");
    Ok(())
}
