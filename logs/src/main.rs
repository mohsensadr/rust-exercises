use std::fs;
use std::io::Error;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR"){
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error>{
    //for the third way with ?, we need to return a Result variant type
    //first way
    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            let error_logs = extract_errors(text_that_was_read.as_str());
            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote errors.txt"),
                Err(reason_write_failed) => {
                    println!("Writing of errors.txt failed: {}", reason_write_failed);
                }
            }
        }
        Err(error) => println!("Error: {}", error)
    }

    // second way
    let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");

    let error_logs = extract_errors(text.as_str());

    fs::write("errors.txt", error_logs.join("\n")).expect("failed to write error.txt");

    // third way
    let text = fs::read_to_string("logs.txt")?;

    let error_logs = extract_errors(text.as_str());

    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())
}
