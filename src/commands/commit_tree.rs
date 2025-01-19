use crate::commands::hash_object::hash_object;

pub(crate) fn commit_tree(
    tree_hash: String,
    parent_hash: Option<String>,
    message: String,
) -> anyhow::Result<()> {
    let mut contents = format!("tree {}\n", tree_hash);
    if let Some(parent_hash) = parent_hash {
        contents.push_str(&format!("parent {}\n", parent_hash));
    }
    contents.push_str(&format!(
        "author Nikolay Karadzhov <nk@gmail.com> 1243040974 -0700\n"
    ));
    contents.push_str(&format!(
        "committer Nikolay Karadzhov <nk@gmail.com> 1243040974 -0700\n"
    ));
    contents.push_str(&format!("\n"));
    contents.push_str(&format!("{message}\n"));

    let (hash, _) = hash_object("commit", true, contents.as_bytes())?;
    println!("{hash}");

    Ok(())
}
