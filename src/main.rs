
use std::{
    env::{
        args
    },
    fs,
    io,
    process,
    path::{
        Path
    }
};


fn main() { 
    process::exit(file_extractor());
       
}


fn file_extractor() ->i32 { 
    // extract all argument in the terminal;
    let env_variable : Vec<_>  = args().collect();
    if env_variable.len() <2 {
        return 1;
    }
    println!("{:?}",env_variable);
    let filename = Path::new(&env_variable[1]);
    println!("{:?}",filename);

    return 1
}
