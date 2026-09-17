use std::io::BufRead;

fn main() {
    println!(r#"
*******************************************************************************
          |                   |                  |                     |
 _________|________________.=""_;=.______________|_____________________|_______
|                   |  ,-"_,=""     `"=.|                  |
|___________________|__"=._o`"-._        `"=.______________|___________________
          |                `"=._o`"=._      _`"=._                     |
 _________|_____________________:=._o "=._."_.-="'"=.__________________|_______
|                   |    __.--" , ; `"=._o." ,-"""-._ ".   |
|___________________|_._"  ,. .` ` `` ,  `"-._"-._   ". '__|___________________
          |           |o`"=._` , "` `; .". ,  "-._"-._; ;              |
 _________|___________| ;`-.o`"=._; ." ` '`."\` . "-._ /_______________|_______
|                   | |o;    `"-.o`"=._``  '` " ,__.--o;   |
|___________________|_| ;     (#) `-.o `"=.`_.--"_o.-; ;___|___________________
____/______/______/___|o;._    "      `".o|o_.--"    ;o;____/______/______/____
/______/______/______/_"=._o--._        ; | ;        ; ;/______/______/______/_
____/______/______/______/__"=._o--._   ;o|o;     _._;o;____/______/______/____
/______/______/______/______/____"=._o._; | ;_.--"o.--"_/______/______/______/_
____/______/______/______/______/_____"=.o|o_.--""___/______/______/______/____
/______/______/______/______/______/______/______/______/______/______/[TomekK]
*******************************************************************************
    "#);

    println!("Welcome to Treasure Island!");
    println!("Your mission is to find the treasure.");

    // First decision point
    println!("Do you want to go left or right?");
    let mut direction = String::new();
    while direction.trim_end() != "left" && direction.trim_end() != "right" {
        direction.clear(); // Clear previous input - must have this
        std::io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");
        println!("Please choose left or right.");
    }

    if direction.trim_end() == "right" {
            println!("The ground gives way to a giant sinkhole, unfortunately, you died.");
        }

    // Continue the story for the "left" option
    if direction.trim_end() == "left" {
        println!("You travel west and encounter a giant river.");
        println!("Do you want to swim or wait?");
        let mut swimming = String::new();
        while swimming.trim_end() != "swim" && swimming.trim_end() != "wait" {
            swimming.clear(); // Clear previous input - must have this
            std::io::stdin()
                .read_line(&mut swimming)
                .expect("Failed to read line");
            println!("Please choose to swim or wait.");
        }

        if swimming.trim_end() == "swim" {
             println!("You have been eaten by alligators.");
        } 
    
        // Continue the story for the "wait" option
        else if swimming.trim_end() == "wait" {
            println!("A ferry has arrived to take you across the river safely.");
            println!("You stumble upon three colorful doors. The first door is in a meadow and is yellow. The second door is red and near a volcano. The third door is blue and near a lake. Which door do you choose?");
            let mut door = String::new();
            while door.trim_end() != "blue" && door.trim_end() != "red" && door.trim_end() != "yellow" {
                door.clear(); // Clear previous input - must have this
                std::io::stdin()
                    .read_line(&mut door)
                    .expect("Failed to read line");
                println!("Please choose blue, red, or yellow.");
            }
    
        if door.trim_end() == "blue" {
            println!("As you approach the door, the vegetation you are walking on sinks. You are in a marsh! You struggle to break free from the mud. Your efforts are in vain as the mud grips you from the waist down pulling you under with every muscle movement. You eventually are swallowed whole by the earth.");
    } 
    
        else if door.trim_end() == "red" {
            println!("You approach the door and you can feel an intense heat growing as you get closer. You open the door and lava spills forth, dissolving your body within seconds.");
    } 
    
        else if door.trim_end() == "yellow" {
            println!("You open the door with excitement for what lies beyond. As the door opens you gasp in disbelief, before you is a sea of gold for as far as the eye can see. You have found the treasure!");
    }}}

    // Pause before exit
    println!("Game Over. Press Enter to exit.");
    let stdin = std::io::stdin();
    let _ = stdin.lock().lines().next();
}