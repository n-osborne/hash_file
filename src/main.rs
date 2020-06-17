use hash_file::*;

fn main() {

    let (b, pb) = check(get_args());
    
    if  b {
        println!("file already present in directory: {:?}", pb);
    } else {
        println!("file not present in the directory");
    }
    
}



