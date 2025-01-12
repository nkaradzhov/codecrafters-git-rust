use crate::object;

pub fn ls_tree(hash: String, name_only: bool) -> anyhow::Result<()> {
    let (dir, file) = hash.split_at(2);
    let full_path = format!(".git/objects/{dir}/{file}");
    let mut reader = object::create_zlib_reader(full_path)?;

    let Ok((kind, _)) = object::read_header(&mut reader) else {
        anyhow::bail!("Incorrect header");
    };

    if kind != "tree" {
        anyhow::bail!("Not a tree object");
    }

    loop {
        let (n, mode_and_name) = object::read_until(&mut reader, 0)?;
        if n == 0 {
            break;
        }
        let Some((_mode, name)) = mode_and_name.split_once(' ') else {
            anyhow::bail!("mode and name");
        };

        let obj_hash = object::read_hash(&mut reader)?;

        if name_only {
            println!("{}", name);
        } else {
            println!("{} {} {}", name, _mode, obj_hash);
        }
    }

    Ok(())
}
