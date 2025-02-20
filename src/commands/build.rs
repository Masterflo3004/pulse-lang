use std::io::{Read, Write};

use std::fs;
use std::path::Path;
use crate::compiler;
use toml;

pub fn build(file:String){

    //opens the given File and reads its content into code
    let path = Path::new(&file);
    let mut file = fs::File::open(path).expect("Could not open the file, please enter a valid file");
    let mut code: String = String::new();
    file.read_to_string(&mut code).expect("An Error Occured while reading the file");

    //compiles the code into one Vector (Vec<String>) and into one String)
    let (libs, rs_code) = compiler::compile(&code);

    //initializes a new cargo project using system calls
    let _cargo = std::process::Command::new("cargo")
        .arg("new")
        //removing the file ending
        .arg(path.file_stem().unwrap().to_str().unwrap())
        //changing the directory to the parent directory
        .current_dir(path.parent().unwrap())
        .output()
        .expect("Could not create a new cargo project");


    // removes all files from the src directory
    fs::remove_dir_all(path.parent().unwrap().join(path.file_stem().unwrap().to_str().unwrap()).join("src")).expect("Could not remove the src directory");
    //creates a new src directory
    fs::create_dir(path.parent().unwrap().join(path.file_stem().unwrap().to_str().unwrap()).join("src")).expect("Could not create the src directory");

    //creates a new src/main.rs file and writes the compiled code into it
    let mut file = fs::File::create(path.parent().unwrap().join(path.file_stem().unwrap().to_str().unwrap()).join("src").join("main.rs")).expect("Could not create the new main.rs file");
    file.write_all(rs_code.as_bytes()).expect("Could not write the compiled code into the new main.rs file");
    //Reads the used Librarys from the Vector and copys their code into src/<lib>.rs
    
    //reads the librarys code from lib/<lib>.toml (the code variable) and writes it into src/<lib>.rs
    for lib in libs{
        //converts the toml file from toml to a HashMap
        let lib_path = path.parent().unwrap().join(path.file_stem().unwrap().to_str().unwrap()).join("lib").join(lib.clone()+".toml");
        //checks if the toml file exists
        if !lib_path.exists(){
            println!("The toml file for the Node {} does not exist", lib);
            break;
        }

        let mut file = fs::File::open(lib_path).expect("Could not open the toml file");
        let mut toml = String::new();
        file.read_to_string(&mut toml).expect("Could not read the toml file");
        let lib_code = toml::from_str::<std::collections::HashMap<String, String>>(&toml).expect("Could not convert the toml file into a HashMap");


        //creates the new file
        let mut file = fs::File::create(path.parent().unwrap().join(path.file_stem().unwrap()
        .to_str().unwrap())
        .join("src").join(lib.clone()+".rs"))
        .expect("Could not create the new file");

        //writes the code into the file
        file.write_all(lib_code.get("code").unwrap().as_bytes()).expect("Could not write the code into the file");

        //reads the cargo dependencies from the used librarys and downloads them
        let dependencies = lib_code.get("dependencies").unwrap();
        let dependencies = dependencies.split(" ");
        for dep in dependencies{
            let _cargo = std::process::Command::new("cargo")
                .arg("add")
                .arg(dep)
                .output()
                .expect("Could not install the dependencies");
        }

        
    }

    //reads the cargo dependencies from the used librarys and downloads them
}