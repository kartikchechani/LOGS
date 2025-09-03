use std::fs;
use std::io::Error;


fn get_error_lines(value: &str) -> Vec<String> {

    let mut error_lines = vec![];

    let split_string = value.split("\n");

    for line in split_string {
        if(line.contains("ERROR")) {
            error_lines.push(line.to_string());
        }
    }

    error_lines
}


fn main() -> Result<(), Error>{

    let read_string = fs::read_to_string("logs.txt")?;
    let vector_of_errors = get_error_lines(&read_string.as_str());
    fs::write("abcd/error.txt", vector_of_errors.join("\n"))?;

    println!("All actions performed successfully");
    Ok(())

    // let mut error_lines = vec![];

    // match fs::read_to_string("logs.txt") {
    //     Ok(value) => {
    //         error_lines = get_error_lines(value.as_str());
    //         println!("File read successfully")
    //     }
    //     Err(error) => {
    //         println!("Error: {}", error);
    //     }
    // }

    // println!("Error lines: {:#?}", error_lines);

}


