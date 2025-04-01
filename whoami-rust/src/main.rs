use std::env;

fn main() {
    // Retrieve the current username using the "USERNAME" environment variable.
    let username = env::var("USERNAME").unwrap_or_else(|_| "Unknown".to_string());
    println!("Current user: {}", username);

    // Retrieve the current working directory.
    match env::current_dir() {
        Ok(path) => println!("Current working directory: {:?}", path),
        Err(e) => eprintln!("Error retrieving current directory: {}", e),
    }

    // Retrieve the hostname using the "COMPUTERNAME" environment variable.
    let hostname = env::var("COMPUTERNAME").unwrap_or_else(|_| "Unknown".to_string());
    println!("Hostname: {}", hostname);
}
