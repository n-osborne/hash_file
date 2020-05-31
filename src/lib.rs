extern crate ring;

use std::env;
use ring::digest::{Context, Digest, SHA256};
use std::io::{ BufReader, Read };
use std::fs::{ File, ReadDir, DirEntry, read_dir };

pub fn sha(file : File) -> Result<Digest, &'static str> {
    let mut reader = BufReader::new(file);
    let mut ctx    = Context::new(&SHA256);
    let mut buf    = [0; 1024];
    let mut count;
    loop {
        count = reader.read(&mut buf).expect("Could not read file");
        if count == 0 { break ; }
        ctx.update(&buf[..count]);
    }
    Ok(ctx.finish())
}

pub fn sha_aux(entry : DirEntry) -> Result<Digest, &'static str> {
    let file = File::open(&entry.path()).expect("Could not open the file");
    sha(file)
    
}

#[derive(Debug)]
pub struct Args {
    pub f1 : File,
    pub rd  : ReadDir,
}
    
pub fn get_args() -> Args {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please provide a file name and a directory path")
    } else {
        Args { f1 : File::open(&args[1]).expect("Could not open file"),
               rd : read_dir(&args[2]).expect("could not open directory"), }
    }
}

pub fn check(args : Args) -> bool {
    let sha_file = sha(args.f1).expect("Could not hash the file");
    let mut res = false;
    for entry in args.rd {
        if let Ok(entry) = entry {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    if sha_file.as_ref() == sha_aux(entry).unwrap().as_ref() {
                        res = true;
                        break;
                    }
                }
            }
        }
    }
    res
}
