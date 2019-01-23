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
    // note dbg!() is a thing now that i can use
    println!("Welcome to WebeWizard's Flashcard game!");
    println!("Use 'help' to show available commands.");
    loop {
        match state {
            AppState::MainMenu => {
                let command = prompt_for_command("> ");
                state = main_menu(command);
            }
            AppState::DeckSelector => {
                let command = prompt_for_command("> ");
                state = deck_selector(command);
            }
            AppState::Game(deck, card_index) => {
                state = card_game(deck, card_index);
            }
        }
        
        // prepare for next input
        io::stdout().flush().unwrap();
    }
}

fn prompt_for_command(msg: &'static str) -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    print!("{}",msg);
    io::stdout().flush().unwrap();
    stdin.read_line(&mut input).expect("Failed to read user input");
    return input.trim_end().to_lowercase();
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
    let command = prompt_for_command("Answer: ");
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
            if index == deck.cards.len()-1 {
                println!("Reached the end of the deck. Returning to main menu.");
                return AppState::MainMenu;
            } else {
                next_index = index+1;
            }
        }
        _ => println!("Incorrect")
    }
    return AppState::Game(deck, next_index);
}