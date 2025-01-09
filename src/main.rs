#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;

use clap::ArgAction;
use clap::Parser;
use clap::Subcommand;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1_smol::Sha1;

static OBJECTS_DIR: &str = ".git/objects";

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

    #[command(arg_required_else_help = true)]
    CatFile {
        #[arg(short, long, action(ArgAction::SetTrue))]
        ppp: bool,
        hash: String,
    },
    #[command(arg_required_else_help = true)]
    HashObject {
        #[arg(short, long, action(ArgAction::SetTrue))]
        write: bool,
        path: String,
    },
}


fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            return init();
        }
        Commands::CatFile { hash, ppp } => {
            return cat_file(hash);
        }
        Commands::HashObject { write, path } => {
            return hash_object(write, path);
        }
    }
}

fn init() -> anyhow::Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(OBJECTS_DIR)?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    Ok(println!("Initialized git directory"))
}

fn cat_file(hash: String) -> anyhow::Result<()> {
    //args[2] is the subcommand, -p, blob, tree, commit, etc
    // cat-file -p -> automatically determine object type
    // let object_hash = &args[3];
    let (dir, file) = hash.split_at(2);

    let full_path = format!("{OBJECTS_DIR}/{dir}/{file}");

    let encoded_contents = fs::read(full_path)?;
    let mut decoder = ZlibDecoder::new(&*encoded_contents);
    let mut contents = String::new();
    decoder.read_to_string(&mut contents)?;

    let mut split = contents.split("\0");
    let contents = split.nth(1).expect("expect file to have contents");
    Ok(print!("{}", contents))
}

fn hash_object(write: bool, path: String) -> anyhow::Result<()> {
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
        encoder.write_all(contents.as_bytes())?;
        let compressed = encoder.finish()?;
        let (dir, file) = hash.split_at(2);
        let path = format!("{OBJECTS_DIR}/{dir}/{file}");
        fs::create_dir_all(format!("{OBJECTS_DIR}/{dir}"))?;
        fs::write(path, compressed)?;
    }
    Ok(println!("{}", String::from(hash)))
}