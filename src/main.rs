
/// rfjnr
use flate2::{
    write::GzEncoder,
    Compression,
};
/// env::args  for command-line arguments
/// fs::File for file operations,
/// io::copy for copying data between streams,
/// io::BufReader for efficient reading of file contents,
/// time::Instant for measuring elapsed time.
use std::{
    env::args,
    fs::File,
    io::{
        copy,
        BufReader,
    },
    time::Instant,
};


fn main() {
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input = BufReader::new(
        File::open(
            args().nth(1).unwrap()// get the first argument from th command line which is the file to copy from
        ).unwrap()
    );
    // create a file to compress
    let output = File::create(
        args().nth(2).unwrap()
    ).unwrap();

    //Creating the Gzip encoder

     let mut encoder =  GzEncoder::new(output,Compression::default());
     let start = Instant::now();

     copy(&mut input,&mut encoder).unwrap();
     let output = encoder.finish().unwrap(); // printing out the zip file after copying
     println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());
    



}
