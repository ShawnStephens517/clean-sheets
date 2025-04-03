use std::env;
use std::error::Error;
<<<<<<< Updated upstream
use std::io::{BufRead, BufReader};
=======
use std::io::{BufRead, BufReader, Write};
>>>>>>> Stashed changes
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error>> {
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

    // Prompt the user for an IP address or default to 8.8.8.8.
    println!("Enter an IP address to ping or press Enter to use the default (8.8.8.8):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let target = if input.trim().is_empty() {
        "8.8.8.8"
    } else {
        input.trim()
    };

    // Call the ping function with the provided target.
    if let Err(e) = ping_me(target) {
        println!("Error: {}", e);
    }
    Ok(())
}

fn ping_me(target: &str) -> Result<(), Box<dyn Error>> {
    println!("Pinging {} for 10 seconds...", target);

    let mut child = Command::new("cmd")
        .args(&["/c", "ping", target, "-n", "10"])
        .stdout(Stdio::piped())
        .spawn()?;

    // Read and print the output from the ping command in real time.
    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    for line in reader.lines() {
        println!("{}", line?);
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(format!("Ping command failed with status: {}", status).into());
    }

    Ok(())
}
