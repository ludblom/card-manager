use scryfall::Card;
use std::sync::mpsc::Sender;

use crate::network::IoEvent;

#[derive(Clone)]
pub struct User {
    username: String,
}

#[derive(Clone)]
pub struct Deck {
    cards: Vec<Card>
}

pub struct App {
    pub saved_decks: Option<Vec<Deck>>,
    // Inputs:
    // input is the string for input;
    // input_idx is the index of the cursor in terms of character;
    // input_cursor_position is the sum of the width of characters preceding the cursor.
    // Reason for this complication is due to non-ASCII characters, they may
    // take more than 1 bytes to store and more than 1 character width to display.
    pub input: Vec<char>,
    pub input_idx: usize,
    pub input_cursor_position: u16,
    pub large_search_limit: u32,
    pub small_search_limit: u32,
    pub user: Option<User>,
    pub is_loading: bool,
    pub io_tx: Option<Sender<IoEvent>>,
    pub dialog: Option<String>,
    pub confirm: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            saved_decks: None,
            input: vec![],
            input_idx: 0,
            input_cursor_position: 0,
            large_search_limit: 40,
            small_search_limit: 3,
            user: None,
            is_loading: false,
            io_tx: None,
            dialog: None,
            confirm: false,
        }
    }
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> App {
        App {
            io_tx: Some(io_tx),
            ..App::default()
        }
    }
}
