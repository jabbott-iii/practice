use std::io;
use std::io::BufRead;
// f64 is a data type that represents a 64-bit floating point number

fn main() {
    println!("Tip Calculator");
    
    // Get user input for total bill
    println!("What is the total bill? $");
    let mut bill = String::new(); 
        io::stdin() 
            .read_line(&mut bill)
            .expect("Failed to read line");

        // Convert bill input to f64
        let decimal_bill: f64 = bill
            .trim_end() 
            .parse() 
            .expect("Please enter a valid number");

            // Get user input for tip percentage
    println!("What percentage would you like to give as a tip? ");
    let mut tip = String::new();
        io::stdin() 
            .read_line(&mut tip) 
            .expect("Failed to read line"); 

            // Convert tip input to f64
        let decimal_tip: f64 = tip
            .trim_end() 
            .parse() 
            .expect("Please enter a valid number"); 

    let bill = decimal_bill; // Shadowing the bill variable
    let tip = decimal_tip;  // Shadowing the tip variable

    let percent = tip / 100.0; // Convert percentage to decimal
    let total = percent * bill; // Calculate total tip amount

    // Get user input for number of people to split the bill
    println!("How many people are you splitting the bill with? Including yourself. ");
    let mut friends = String::new();
        io::stdin() 
            .read_line(&mut friends) 
            .expect("Failed to read line"); 

        // Convert friends input to f64
        let decimal_friends: f64 = friends
            .trim_end() 
            .parse() 
            .expect("Please enter a valid number"); 

    let friends = decimal_friends; // Shadowing the friends variable
    let final_amount = total / friends; // Calculate individual share of the tip

    println!("Individual share of the tip is ${:.2}", final_amount);

    println!("Program finished. Press Enter to exit...");
    let stdin = io::stdin(); // Get standard input
    let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}