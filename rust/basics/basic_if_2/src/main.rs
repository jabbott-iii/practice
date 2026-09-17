use std::io::BufRead;

fn main() {
    println!("Welcome to the rollercoaster!");

    // Get user input for height
    println!("How tall are you in inches?");
    let mut height = String::new();
        std::io::stdin()
            .read_line(&mut height)
            .expect("Failed to read line");

            // Convert height input to i8
        let height: i8 = height
            .trim_end()
            .parse()
            .expect("Please enter a valid number");
    
    if height >= 60 {
        println!("You may ride!");
    } else {
        println!("You will need to wait until you are taller!");
        return; // Exit the program if the user is not tall enough
    }

    // Get user input for age
        println!("How old are you?");
        let mut age = String::new();
            std::io::stdin()
                .read_line(&mut age)
                .expect("Failed to read line");
            
            // Convert age input to i8
            let age: i8 = age
                .trim_end()
                .parse()
                .expect("Please enter a valid number");

                // Determine ticket price based on age
        let mut bill: i8 = 0;
        if age >= 45 && age <= 55 {
            println!("Free midlife crisis ticket!");
            bill += 0;
        } else if age >= 18 {
            println!("Adult tickets will be 10 dollars");
            bill += 10;
        } else if age >= 14 {
            println!("Youth tickets will be 7 dollars");
            bill += 7;
        } else {
            println!("Child tickets will be 5 dollars");
            bill += 5;
        }

        println!("Would you like your photo taken? Type Y for yes and N for no.");
        let mut wants_photo = String::new();
            std::io::stdin()
                .read_line(&mut wants_photo)
                .expect("Failed to read line");


        if wants_photo.trim_end() == "Y" {
            bill += 3;
            // += is the same as bill = bill + 3
        }

        println!("Your final bill is ${}", bill);

        println!("Program finished. Press Enter to exit...");
        let stdin = std::io::stdin(); // Get standard input
        let _ = stdin.lock().lines().next(); // Wait for user to press Enter
}