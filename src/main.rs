#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Write;

use anyhow::anyhow;
use anyhow::Context;
use clap::ArgAction;
use clap::Parser;
use clap::Subcommand;
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

    CatFile {
        #[arg(name("p"), short, long, action(ArgAction::SetTrue))]
        p: bool,
        hash: String,
    },
    HashObject {
        #[arg(short, long, action(ArgAction::SetTrue))]
        write: bool,
        path: String,
    },
    LsTree {
        #[arg(short, long, action(ArgAction::SetTrue))]
        name_only: bool,
        hash: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Commands::Init => init(),
        Commands::CatFile { hash, p: _ } => cat_file(hash),
        Commands::HashObject { write, path } => hash_object(write, path),
        Commands::LsTree { name_only, hash } => ls_tree(hash, name_only),
    }
}

fn init() -> anyhow::Result<()> {
    fs::create_dir(".git").context("Could not create .git directory")?;
    fs::create_dir(OBJECTS_DIR).context(format!("Could not create {OBJECTS_DIR} directory"))?;
    fs::create_dir(".git/refs").context("Could not create .git/refs directory")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n").context("Could not write to .git/HEAD")?;
    println!("Initialized git directory");
    Ok(())
}

fn cat_file(hash: String) -> anyhow::Result<()> {
    let (dir, file) = hash.split_at(2);

    let full_path = format!("{OBJECTS_DIR}/{dir}/{file}");

    let mut reader = object::create_zlib_reader(full_path)?;
    let contents = object::read_to_string(&mut reader)?;

    let mut split = contents.split("\0");
    // let contents = split.nth(1).expect("expect file to have contents");
    let contents = split.nth(1).ok_or_else(|| anyhow!("No Contents!"))?;
    print!("{}", contents);
    Ok(())
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
        encoder
            .write_all(contents.as_bytes())
            .context("Could not compress file contents")?;
        let compressed = encoder
            .finish()
            .context("Could not compress file contents")?;
        let (dir, file) = hash.split_at(2);
        let path = format!("{OBJECTS_DIR}/{dir}/{file}");
        let directory = format!("{OBJECTS_DIR}/{dir}");
        fs::create_dir_all(&directory)
            .context(format!("Could not create directory {directory}"))?;
        fs::write(&path, compressed).context(format!("Could not write file {path}"))?;
    }
    println!("{}", hash);
    Ok(())
}

fn ls_tree(hash: String, name_only: bool) -> anyhow::Result<()> {
    let (dir, file) = hash.split_at(2);
    let full_path = format!("{OBJECTS_DIR}/{dir}/{file}");
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

mod object {
    use std::{
        ffi::CStr,
        fs::{self, File},
        io::{BufRead, BufReader, Read},
        path::Path,
    };

    use flate2::read::ZlibDecoder;

    type ObjectReader = BufReader<ZlibDecoder<File>>;

    pub(crate) fn create_zlib_reader<P: AsRef<Path>>(path: P) -> anyhow::Result<ObjectReader> {
        let file = fs::File::open(path)?;
        let z = ZlibDecoder::new(file);
        Ok(BufReader::new(z))
    }
    pub(crate) fn read_header(r: &mut ObjectReader) -> anyhow::Result<(String, String)> {
        let mut buf = Vec::new();
        r.read_until(0, &mut buf)?;
        let header = CStr::from_bytes_until_nul(&buf)?.to_str()?;
        let Some((kind, size)) = header.split_once(' ') else {
            anyhow::bail!("Incorrect header");
        };
        Ok((kind.to_string(), size.to_string()))
    }
    pub(crate) fn read_hash(r: &mut ObjectReader) -> anyhow::Result<String> {
        let mut buf = [0; 20];
        r.read_exact(&mut buf)?;
        let hash = hex::encode(buf);
        Ok(hash)
    }
    pub(crate) fn read_until(r: &mut ObjectReader, byte: u8) -> anyhow::Result<(usize, String)> {
        let mut buf = Vec::new();
        let n = r.read_until(byte, &mut buf)?;
        if n == 0 {
            return Ok((n, "".to_string()));
        }
        let s = CStr::from_bytes_until_nul(&buf)?.to_str()?;
        let s = s.to_string();

        Ok((n, s))
    }

    pub(crate) fn read_to_string(r: &mut ObjectReader) -> anyhow::Result<String> {
        let mut s = String::new();
        r.read_to_string(&mut s)?;
        Ok(s)
    }
}
