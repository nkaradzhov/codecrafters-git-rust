#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::Path;

use clap::ArgAction;
use clap::Parser;
use clap::Subcommand;
use commands::{
    cat_file::cat_file, hash_object::hash_object, init::init, ls_tree::ls_tree,
    write_tree::write_tree,
};

mod commands;
mod object;

#[derive(Parser, Debug)]
#[command(name = "git")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Init,
    CatFile {
        #[arg(name("p"), short, long, action(ArgAction::SetTrue))]
        p: bool,
        hash: String,
    },
    HashObject {
        #[arg(short, long, action(ArgAction::SetTrue))]
        write: bool,
        path: String,
        #[arg(short('t'), default_value("blob"))]
        object_type: String,
    },
    LsTree {
        #[arg(short, long, action(ArgAction::SetTrue))]
        name_only: bool,
        hash: String,
    },
    WriteTree,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init => init(),
        Commands::CatFile { hash, p: _ } => cat_file(hash),
        Commands::HashObject {
            write,
            path,
            object_type,
        } => {
            let path = Path::new(&path);
            let contents = fs::read_to_string(path).expect("should point to a valid file path");
            let (hash, _) = hash_object(&object_type, write, contents.as_bytes())?;
            println!("{}", hash);
            Ok(())
        }
        Commands::LsTree { name_only, hash } => ls_tree(hash, name_only),
        Commands::WriteTree => write_tree(),
    }
}
