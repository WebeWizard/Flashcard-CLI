use std::io;
use std::io::prelude::*;

extern crate lib_flashcard;
use lib_flashcard::{deck,card};

enum AppState {
    MainMenu,
    DeckSelector,
    Game(deck::Deck, usize)
}

fn main() {
    let mut state = AppState::MainMenu;

    println!("Welcome to WebeWizard's Flashcard game!");
    println!("Use 'help' to show available commands.");
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        match state {
            AppState::MainMenu => {
                print!("> ");
                io::stdout().flush().unwrap();
                stdin.read_line(&mut input).expect("Failed to read user input");
                let command = input.trim_end().to_lowercase();
                state = main_menu(command);
            }
            AppState::DeckSelector => {
                print!("> ");
                io::stdout().flush().unwrap();
                stdin.read_line(&mut input).expect("Failed to read user input");
                let command = input.trim_end().to_lowercase(); // assuming no question/answer ends in whitespace
                state = deck_selector(command);
            }
            AppState::Game(deck, card_index) => {
                state = card_game(deck, card_index);
            }
        }
        
        // prompt for next input
        input.clear();
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

fn main_menu(command: String) -> AppState {
    match command.as_str() {
        "help" => {
            println!("~ Available Commands ~");
            println!("help - displays this menu");
            println!("decks - lists all available decks");
            println!("start - launch the deck selector");
            println!("reset - resets scores for all decks");
            println!("exit - quits this program");
            return AppState::MainMenu;
        }
        "start" => {
            println!("Enter the name of a deck to start");
            // launch the deck selector
            return AppState::DeckSelector;
        }
        "exit" => {
            // quit the application
            println!("Exiting the application.");
            std::process::exit(0);
        }
        // TODO: Need a command to list available decks
        // default if command is unrecognized
        _ => {
            println!("Not a valid command:  '{}'",command);
            return AppState::MainMenu;
        }
    }
}

fn deck_selector(requested_deck: String) -> AppState {
    let path = &format!("./resources/{}",requested_deck);
    match deck::Deck::read_from_yaml(path) {
        Ok(deck) => {
            println!("Deck Found!");
            println!("Enter 'X' to leave deck");
            println!("Enter '<' to go to previous card");
            println!("Enter '>' to go to next card");
            println!("Enter '^' to see answer");
            return AppState::Game(deck, 0);
        }
        Err(_) => {
            println!("Deck {} was not found", requested_deck);
            return AppState::DeckSelector;
        }
    }
}

fn card_game(deck: deck::Deck, index: usize) -> AppState {
    // TODO: track state/score of deck.
    // make it part of the main state
    let card: &card::Card = &deck.cards[index];
    println!("Question: {}", deck.cards[index].question);
    print!("Answer: ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read user input");
    let command = input.trim_end().to_lowercase();
    let mut next_index = index;
    match command.as_str() {
        "x" => {
            println!("Exiting deck");
            return AppState::MainMenu;
        }
        "<" => next_index = if index == 0 {deck.cards.len()-1} else {index-1},
        ">" => next_index = if index == deck.cards.len()-1 {0} else {index+1},
        "^" => {
            println!("Answer:  {}",card.answer);
            io::stdout().flush().unwrap();
        }
        _ if command == card.answer => {
            println!("Correct! The answer is:  {}", card.answer);
        }
        _ => println!("Incorrect")
    }
    return AppState::Game(deck, next_index);
}