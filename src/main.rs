use hash_file::*;

fn main() {

    
    match get_args() {
        Err(err) =>  println!("{}", err),
        Ok(args) => {
            
            match check(args) {
                Ok((b, pb)) => {
    
                    if  b {
                        println!("file already present in directory: {:?}", pb);
                    } else {
                        println!("file not present in the directory");
                    }
                }
            Err(err) => println!("{}", err)
            }
        }
    }
    
}



