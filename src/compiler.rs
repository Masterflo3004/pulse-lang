use std::collections::HashMap;
pub fn create_code (line: &String){

}



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
    let mut lib_l:Vec<String>;
    let mut var_l: HashMap<String, bool> = HashMap::new();
    let mut num = 1;

    //4
    for x in &line_l{
        let statement = x.split("/").collect::<Vec<&str>>();


        if statement.iter().count() != 3{
            println!("There was an error at line {} please check if you declared every in and output or if oyu wrote _ if none is required", num);
        }
        let lib = statement[1].to_string(); //gets the used Node of the command and puts its name into the lib
        lib_l.push(lib);





        num += 1;
    }

    return (lib_l, "".to_string());
}