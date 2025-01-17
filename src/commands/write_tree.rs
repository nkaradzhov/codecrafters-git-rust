use std::{
    fs::{self},
    path::{self, Path},
};

use crate::commands::hash_object;

pub fn write_tree() -> anyhow::Result<()> {
    let (hash, _) = write(path::Path::new("."))?;
    println!("{}", hash);
    Ok(())
}

fn write(path: &Path) -> anyhow::Result<(String, [u8; 20])> {
    if path.is_dir() {
        let mut contents: Vec<u8> = Vec::new();
        let mut entries = fs::read_dir(&path)?
            .filter_map(|entry| entry.ok())
            .collect::<Vec<_>>();
        entries.sort_by_key(|dir| dir.file_name());

        for entry in entries {
            let path = entry.path();
            if path.starts_with("./.git") {
                continue;
            }
            let (_, sha) = write(&entry.path())?;
            let (mode, _) = if entry.path().is_dir() {
                (040000, "tree".to_string())
            } else {
                (100644, "blob".to_string())
            };

            let Ok(file_name) = entry.file_name().into_string() else {
                anyhow::bail!("Could not convert OsString to String")
            };

            let header = format!("{} {}\0", mode, file_name);
            let header = header.as_bytes();
            contents.extend_from_slice(header);
            contents.extend_from_slice(&sha);
        }

        return hash_object::hash_object("tree", true, &contents);
    } else {
        let contents = fs::read_to_string(path).expect("should point to a valid file path");
        return hash_object::hash_object("blob", true, contents.as_bytes());
    }
}
