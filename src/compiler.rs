use std::collections::HashMap;



pub fn compile (code: &String) -> (Vec<String>, String){

    //STEPS:
    //1.split the code into single lines and convert them to a vector
    //2.create the vector for the librarys
    //3. create a Hashmap for the declared variables
    //4. creates a list for the ready code lines

    //5.loop through all lines and count the lines
    //5.1convert the line from a &&str into a Vector
    //5.2check if the line has enough parts (input/node/output)
    //5.3adds the node to the list of required librarys


    //5.4splits input variables into vector
    //5.5checks if input variables are declared 
    //5.6splits output variables into a Vector
    //5.7adds output variables to the variable vector
    //5.8converts the line into a rust statement and adds it to the code list

    //6creates the finished code


    //splits the code at every \n
    let lines = code.split("\n");
    let line_l = lines.collect::<Vec<&str>>();
    //created the vector for the libs
    let mut lib_l:Vec<String> = Vec::new();
    //creates the hashmap for the variables
    let mut var_l: HashMap<String, bool> = HashMap::new();

    //creates the vector for the code
    let mut code_l: Vec<String> = Vec::new();

    code_l.push("fn main() {".to_string());

    //creates the counter for the lines
    let mut num = 1;

    //4
    for x in &line_l{
        if x.to_string().starts_with("#"){
            continue;
        }


        let statement: Vec<&str> = x.splitn(3, '!').collect();


        if statement.iter().count() != 3{
            print!("{}", statement.iter().count());
            println!("There was an error at line {} please check if you declared every in and output or if you wrote _ if none is required", num);
        }
        let lib = statement[1].to_string(); //gets the used Node of the command and puts its name into the lib
        lib_l.push(lib.clone()); //adds the lib to the lib list

        let input = statement[0].split(":").collect::<Vec<&str>>();
        for x in &input{
            if *x != "_"{
                if var_l.contains_key(*x) != true{
                    println!("there was an error splitting argument {}", num);
                }
            }
        }

        let output = statement[2].split(":").collect::<Vec<&str>>();
        for x in &output{
            var_l.insert(x.to_string(), true);
        }

        //creates the code line
        let mut code_line = "".to_string();
        code_line = code_line + "let ";
        for x in &output{
            //NOTE: add the check if the types are matching
            code_line = code_line + x + ",";
        }
        code_line = code_line + " = ";

        //adds the function lib from dependency lib 
        code_line = code_line + &lib + "::" + &lib + "(";
        for x in &input{
            code_line = code_line + x + ",";
        }
        //remove the last ,
        code_line = code_line.trim_end_matches(',').to_string();

        code_line = code_line + ");";

        //adds the code line to the code list
        code_l.push(code_line);




        num += 1;
    }
    code_l.push("}".to_string());

    //inserts all the required librarys
    for x in &lib_l{
        code_l.insert(0, "mod ".to_string() + x + ";");
    }

    //imports the librarys
    for x in &lib_l{
        code_l.insert(lib_l.iter().count(), "use ".to_string() + x);
    }

    return (lib_l, code_l.join("\n"));
}