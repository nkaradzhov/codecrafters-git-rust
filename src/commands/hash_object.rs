use std::{fs, io::Write};

use anyhow::Context;
use flate2::{write::ZlibEncoder, Compression};
use sha1_smol::Sha1;

type Sha = [u8; 20];
pub fn hash_object(
    object_type: &str,
    write: bool,
    contents: &[u8],
) -> anyhow::Result<(String, Sha)> {
    validate_object_type(object_type)?;

    let size = contents.len();
    let header = format!("{} {}\0", object_type, size);
    let header = header.as_bytes();

    let mut hasher = Sha1::new();
    hasher.update(header);
    hasher.update(contents);

    let digest = hasher.digest();
    let sha = digest.clone().bytes();
    let hash = digest.to_string();

    if write {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(header).context("write header")?;
        encoder
            .write_all(contents)
            .context("Could not compress file contents")?;
        let compressed = encoder
            .finish()
            .context("Could not compress file contents")?;
        let (dir, file) = hash.split_at(2);
        let path = format!(".git/objects/{dir}/{file}");
        let directory = format!(".git/objects/{dir}");
        fs::create_dir_all(&directory)
            .context(format!("Could not create directory {directory}"))?;
        // for debugging purposes, write content uncompressed
        // fs::write(&path, contents).context(format!("Could not write file {path}"))?;
        fs::write(&path, compressed).context(format!("Could not write file {path}"))?;
    }
    Ok((hash, sha))
}

fn validate_object_type(object_type: &str) -> anyhow::Result<()> {
    if object_type != "blob" && object_type != "commit" && object_type != "tree" {
        anyhow::bail!(format!("Invalid object type {object_type}"));
    }
    Ok(())
}
