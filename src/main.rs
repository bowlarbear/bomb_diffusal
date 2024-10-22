use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        println!("Recruit, welcome to bomb diffusal school!");
        thread::sleep(Duration::from_secs(3));      // Randomly select the correct wire (from 1 to 5)

        let correct_wire = rand::thread_rng().gen_range(1..=5);
        let mut attempts = 0;
        let max_attempts = 2;        
        while attempts < max_attempts {
            println!("There are 5 wires in front of you recruit. Which wire do you want to cut? (Hint: Enter 1-5):");    

            // Read user input
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");     

            // Parse input to a number
            let guess: u32 = match guess.trim().parse() {
                Ok(num) if num >= 1 && num <= 5 => num, // Make sure the input is within the valid range
                _ => {
                    println!("Please enter a valid wire number (1-5) recruit.");
                    thread::sleep(Duration::from_secs(1)); // Add a delay after invalid input
                    continue;
                }
            };            
            
            // Check if the guess is correct
            if guess == correct_wire {
                println!("You diffused the bomb and earned a promotion!");
                break;
            } else {
                attempts += 1;
                println!("Wrong wire recruit! You only have 1 attempt remaining!.");
                thread::sleep(Duration::from_secs(3)); // Add a delay after wrong guess
            }
        }        
        
        // If two failed attempts, the bomb explodes
        if attempts == max_attempts {
            println!("BOOM! Oh dear. You are dead.");
            thread::sleep(Duration::from_secs(3)); // Add a 2-second delay for dramatic effect
        }        
        
        // Restart the game
        println!("Next recruit...Step on in!\n");
        thread::sleep(Duration::from_secs(3)); // Add a 2-second delay before restarting
    }
}