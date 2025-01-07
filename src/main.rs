#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;
use std::io::Write;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1_smol::Sha1;

static OBJECTS_DIR: &str = ".git/objects";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(OBJECTS_DIR).unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        //args[2] is the subcommand, -p, blob, tree, commit, etc
        let object_hash = &args[3];
        let (dir, file) = object_hash.split_at(2);

        let full_path = format!("{OBJECTS_DIR}/{dir}/{file}");

        let encoded_contents = fs::read(full_path).unwrap();
        let mut decoder = ZlibDecoder::new(&*encoded_contents);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents).unwrap();

        let mut split = contents.split("\0");
        let contents = split.nth(1).expect("expect file to have contents");
        print!("{}", contents);
    } else if args[1] == "hash-object" {
        let should_write = args[2] == "-w";
        let path = &args[3];
        let object_type = "blob";

        let contents = fs::read_to_string(path).expect("should point to a valid file path");
        let size = contents.len();

        let contents = format!("{} {}\0{}", object_type, size, contents);
        let mut hasher = Sha1::new();
        hasher.update(contents.as_bytes());
        let hash = hasher.digest().to_string();

        if should_write {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(contents.as_bytes()).unwrap();
            let compressed = encoder.finish().unwrap();
            let (dir, file) = hash.split_at(2);
            let path = format!("{OBJECTS_DIR}/{dir}/{file}");
            fs::create_dir_all(format!("{OBJECTS_DIR}/{dir}")).unwrap();
            fs::write(path, compressed).unwrap();
        }

        println!("{}", String::from(hash));
    } else {
        println!("unknown command: {}", args[1])
    }
}
