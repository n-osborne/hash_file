use hash_file::*;

fn main() {

    if check(get_args()) {
        println!("file already present in directory");
    } else {
        println!("file not present in the directory");
    }
    
}



