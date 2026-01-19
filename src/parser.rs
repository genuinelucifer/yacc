use std::error::Error;

pub fn run(_: Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("Ran parser!");
    Ok(())
}
