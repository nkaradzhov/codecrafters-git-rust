#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::Read;

use flate2::bufread::ZlibDecoder;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir(".git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
        println!("Initialized git directory")
    } else if args[1] == "cat-file" {
        //args[2] is the subcommand, -p, blob, tree, commit, etc
        let object_hash = &args[3];
        let (dir, file) = object_hash.split_at(2);

        let full_path = format!(".git/objects/{dir}/{file}");

        let encoded_contents = fs::read(full_path).unwrap();
        let mut decoder = ZlibDecoder::new(&*encoded_contents);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents).unwrap();

        let mut split = contents.split("\0");
        let contents = split.nth(1).expect("expect file to have contents");
        println!("{}", contents);

    } else {
        println!("unknown command: {}", args[1])
    }
}
