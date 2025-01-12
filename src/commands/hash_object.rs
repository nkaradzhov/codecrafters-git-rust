use std::{fs, io::Write};

use anyhow::Context;
use flate2::{write::ZlibEncoder, Compression};
use sha1_smol::Sha1;

pub fn hash_object(write: bool, path: String) -> anyhow::Result<()> {
    // let should_write = args[2] == "-w";
    // let path = &args[3];
    let object_type = "blob";

    let contents = fs::read_to_string(path).expect("should point to a valid file path");
    let size = contents.len();

    let contents = format!("{} {}\0{}", object_type, size, contents);
    let mut hasher = Sha1::new();
    hasher.update(contents.as_bytes());
    let hash = hasher.digest().to_string();

    if write {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(contents.as_bytes())
            .context("Could not compress file contents")?;
        let compressed = encoder
            .finish()
            .context("Could not compress file contents")?;
        let (dir, file) = hash.split_at(2);
        let path = format!(".git/objects/{dir}/{file}");
        let directory = format!(".git/objects/{dir}");
        fs::create_dir_all(&directory)
            .context(format!("Could not create directory {directory}"))?;
        fs::write(&path, compressed).context(format!("Could not write file {path}"))?;
    }
    println!("{}", hash);
    Ok(())
}
