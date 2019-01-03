use std::io;
use std::io::prelude::*;

fn main() {
    println!("Welcome to WebeWizard's Flashcard game!");
    println!("Use 'help' to show available commands.");
    print!("> ");
    io::stdout().flush().unwrap();
    // wait for user input
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let command = line.unwrap().to_lowercase();
        match command.as_str() {
            "help" => {
                println!("~ Available Commands ~");
                println!("help - displays this menu");
                println!("decks - lists all available decks");
                println!("start <deck> - launch the selected deck");
                println!("reset - resets scores for all decks");
                println!("quit - quits this program");
            }
            "decks" => {
                // list all available decks
            }
            // default if command is unrecognized
            _ => println!("Not a valid command")
        }
        // prompt for next input
        print!("> ");
        io::stdout().flush().unwrap();
    }

    // top level commands:
    // help , lists available commands
    // decks
    // start <deck>
    // quit
    // reset , prompt the user if they are sure (y/n) , if 'y', then reset score for all decks

    // how the game works...
    // display question to user
    // wait for command or answer
    // if answer correct, update/save score and move to next card
    // if all cards have been answered, tell the user great job and ask if they want to reset
    // if answer incorrect, let the user know, wait for another answer
    // if 'help' , then show available commands
    // if 'show' , then display the answer
    // if 'hide' , then show the question
    // if 'next' , then show the next card
    // if 'prev' , then show the previous card
    // if 'exit' , then exit deck and return to main loop
    // if 'reset' , prompt the user if they are sure (y/n) , if 'y', then reset score for this deck

}
