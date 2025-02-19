use std::io::Read;

use std::fs;
use std::path::Path;
use crate::compiler;

pub fn build(file:String){

    //opens the given File and reads its content into code
    let path = Path::new(&file);
    let mut file = fs::File::open(path).expect("Could not open the file, please enter a valid file");
    let mut code: String = String::new();
    file.read_to_string(&mut code).expect("An Error Occured while reading the file");

    //compiles the code into one Vector (Vec<String>) and into one String)
    let (libs, rs_code) = compiler::compile(&code)

    //initializes a new cargo project

    //writes the the String into src/main

    //Reads the used Librarys from the Vector and copies the files into /src

    //reads the cargo dependencies from the used librarys and downloads them
}