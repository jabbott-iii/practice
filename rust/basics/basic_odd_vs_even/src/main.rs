use std::io::BufRead;

fn main() {
    println!("Odd or Even Check");
    println!("Please enter a number: ");
    // Get user input
    let mut userdata = String::new();
        std::io::stdin()
            .read_line(&mut userdata)
            .expect("Failed to read line");
            
        // Convert input to i32    
        let userdata: i32 = userdata.trim()
            .parse()
            .expect("Please enter a valid number");

    if userdata % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    // Pause before exit
    println!("Press Enter to exit.");
    let stdin = std::io::stdin();
    let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}