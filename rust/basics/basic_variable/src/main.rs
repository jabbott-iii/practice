use std::io;
use std::io::BufRead;

fn main() {
    println!("Welcome to the band name generator.");
    println!("What city were you born in?");

    // Get user input for city name
    let mut city: String = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read line");

            // Get user input for pet name
    println!("What was the name of your first pet?");
    let mut pet: String = String::new();
        io::stdin()
            .read_line(&mut pet)
            .expect("Failed to read line");

    println!("Your band name is {}{}", city.trim(), pet.trim()); // Concatenate and display the band name


    println!("Program finished. Press Enter to exit...");
    let stdin = io::stdin(); // Get standard input
    let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}