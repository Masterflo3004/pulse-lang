mod commands;
mod compiler;
fn main() {
    let action = std::env::args().nth(1).unwrap_or("none".to_string());
    let file = std::env::args().nth(2).unwrap_or("none".to_string());

    if action == "help"{
        commands::help::help();
    } else if action == "build"{
        if file == "none"{
            println!("Please enter a Filename!");
            println!("pulse-lang build <filename>");
        } else {
            commands::build::build(file);
        }
        
    } else if action == "run"{
        if file == "none"{
            println!("Please enter a Filename!");
            println!("pulse-lang run <filename>");
        } else {
            commands::run::run(file);
        }

    }
}
