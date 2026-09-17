/* 
Figure out the amount they need to pay based on choices
S = 15 / M = 20 / L = 25 / Spepperoni = +2 / M+Lpepperoni = +3 / Any extra cheese = +1
 */
use std::io::BufRead;

fn main() {
    println!("Welcome to rust pizzia deliveries!");
    println!("What size pizzia do you want? S, M, or L: ");

    let mut bill: i16 = 0; // Variable to hold the total bill amount
    
    // Get user input for pizzia size
    
    let mut size = String::new();
    while size.trim_end() != "S" && size.trim_end() != "M" && size.trim_end() != "L" {
        size.clear(); // Clear previous input - must have this
        std::io::stdin()
            .read_line(&mut size)
            .expect("Failed to read line");
        println!("Please specify L, M, or S.");
    }

            // Determine the base price based on size
            if size.trim_end() == "S" {
                println!("You have selected a small pizzia. That will be $15.");
                bill += 15;
            } else if size.trim_end() == "M" {
                println!("You have selected a medium pizzia. That will be $20.");
                bill += 20;
            } else if size.trim_end() == "L" {
                println!("You have selected a large pizzia. That will be $25.");
                bill += 25;
            } else {
                println!("Please specify L, M, or S.");
            }
    
    // Get user input for pepperoni addition
    println!("Do you want pepperoni? Y or N: ");
    let mut pepperoni = String::new();
    while pepperoni.trim_end() != "Y" && pepperoni.trim_end() != "N" {
        pepperoni.clear(); // Clear previous input - must have this
        std::io::stdin()
            .read_line(&mut pepperoni)
            .expect("Failed to read line");
        println!("Please specify Y or N.");
    }

            // Determine the price addition based on pepperoni choice and size
            if pepperoni.trim_end() == "Y" {
                if size.trim_end() == "S" {
                    println!("You have added pepperoni to your small pizzia. That will be an extra $2.");
                    bill += 2;
                } else if size.trim_end() == "M" || size.trim_end() == "L" {
                    println!("You have added pepperoni to your medium or large pizzia. That will be an extra $3.");
                    bill += 3;
                }
            } else if pepperoni.trim_end() == "N" {
                println!("You have not added pepperoni to your pizzia.");
            } else {
                println!("Please specify Y or N.");
            }

    // Get user input for extra cheese addition
    println!("Do you want extra cheese? Y or N: ");
    let mut extra_cheese = String::new();
    while extra_cheese.trim_end() != "Y" && extra_cheese.trim_end() != "N" {
        extra_cheese.clear(); // Clear previous input - must have this
        std::io::stdin()
            .read_line(&mut extra_cheese)
            .expect("Failed to read line");
        println!("Please specify Y or N.");
    }

        // Determine the price addition based on extra cheese choice
            if extra_cheese.trim_end() == "Y" {
                println!("You have added extra cheese to your pizzia. That will be an extra $1.");
                bill += 1;
            } else if extra_cheese.trim_end() == "N" {
                println!("You have not added extra cheese to your pizzia.");
            } else {
                println!("Please specify Y or N.");
            }

    println!("Your total price is ${}", bill);

    println!("Program finished. Press Enter to exit...");
    let stdin = std::io::stdin(); // Get standard input
    let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}