use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let _file_path = &args[2];

    match query.as_str() {
        "start" => println!("started"),
        "add" => println!("adding"),
        _ => println!("Please select a valid option"),
    }

    println!("Hello, world!");
}
