use std::fs;
use std::io::Error;

fn divide(a: f64, b: f64) -> Result<f64,Error>{
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    }
    else{
        Ok(a/b)
    }
}

fn validate_email(email: String) -> Result<(),Error> {
    if email.contains("@") {
        Ok(())
    }
    else {
        Err(Error::other("email must contain @"))
    }
}
fn main() {
    //let text = fs::read_to_string("logs.txt");
    //println!("{:#?}", text);
    
    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("{}", what_went_wrong);
        }
    }

    match validate_email(String::from("gmail.com")){
        Ok(..) => println!("email is valid"),
        Err (failure_reason) => {
            println!("{}", failure_reason)
        }
    }
}
