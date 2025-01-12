use crate::object;
use anyhow::anyhow;

pub fn cat_file(hash: String) -> anyhow::Result<()> {
    let (dir, file) = hash.split_at(2);

    let full_path = format!(".git/objects/{dir}/{file}");

    let mut reader = object::create_zlib_reader(full_path)?;
    let contents = object::read_to_string(&mut reader)?;

    let mut split = contents.split("\0");
    // let contents = split.nth(1).expect("expect file to have contents");
    let contents = split.nth(1).ok_or_else(|| anyhow!("No Contents!"))?;
    print!("{}", contents);
    Ok(())
}
