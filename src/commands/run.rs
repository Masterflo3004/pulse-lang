use crate::commands;

pub fn run(file:String){
    //builds the project
    commands::build::build(file);

    //runs the project
    
}