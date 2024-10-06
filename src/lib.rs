use std::thread;
use std::sync::mpsc;
use std::fmt;
use rand::prelude::*;


#[derive(Debug)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
//Hearts, Diamonds, Clubs, Spades

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
            Suit::Spades => "Spades",
        };
        write!(f, "{}", suit)
    }
}
//implements Display to Suit enum to print
//May ditch later on when displaying values or pictures in place

#[derive(Debug)]
pub struct Card {
    pub value: u8,
    pub suit: Suit,
}
//Contains(value<u8>, suit<Suit>)

impl Card {

    fn new(value: u8, suit: u8) -> Card {
        let mut temp_value = value;
        let temp_suit;

        match suit {
            1 => temp_suit = Suit::Hearts,
            2 => temp_suit = Suit::Diamonds,
            3 => temp_suit = Suit::Clubs,
            _ => temp_suit = Suit::Spades,
        }

        if value > 10  && value < 14{
            temp_value = 10;
        } else if value == 14 {
            temp_value = 11
        }

        Card {
            value: temp_value,
            suit: temp_suit,
        }
    }

    fn set_to_one(&mut self) {
        self.value = 1;
    }
}
//Methods: new(), set_to_one()

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

pub struct Deck {
    cards: Vec<Card>,
}
//Contains(Vec<Card>)

impl Deck {
    pub fn new() -> Self {
        let mut new_deck = Deck { cards: Vec::new() };
        let (tx, rx) = mpsc::channel();

        for each in 1..=4 {
            let tx = tx.clone();
            thread::spawn(move || {
                for i in 2..=14 {
                    tx.send(Card::new(i, each)).unwrap();
                }
            });
        };
        drop(tx);
    
        for received in rx {
            new_deck.cards.push(received)
        }
    new_deck
    }

    pub fn shuffle(&mut self)-> &mut Self {
        for _i in  0..1000 {
            let from_loc = rand::thread_rng().gen_range(0..52);
            let to_loc = rand::thread_rng().gen_range(0..52);
            self.cards.swap(from_loc, to_loc);
        };
        self
    }

    pub fn deal(&mut self, player: &mut Player) {
        let new_card = self.cards.pop().unwrap();
        let score = player.get_score();
        if score > 21 {
            player.hand.iter_mut().for_each(|x| if x.value == 11 {
                x.set_to_one();
            })
        }
        player.hand.push(new_card);
        self.cards.pop();
    }
}
//Methods: new(), shuffle(), deal()

pub struct Player {
    pub hand: Vec<Card>,
}
//Contains: hand(Vec<Card>)

impl Player {
    pub fn new() -> Self {
        Player { hand: Vec::new() }
    }

    pub fn print_hand(&self) {
        for card in &self.hand {
            print!("{} of {}, ", card.value, card.suit);
        }
        println!();
    }

    pub fn get_score(&self) -> u8 {
        let mut score = 0;
        self.hand.iter().for_each(|x| score += x.value);
        score
    }
}
//Methods: new, print_hand, get_score

pub struct GameState {
    pub deck: Deck,
    pub player: Player,
    pub dealer: Player,
}

impl GameState {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        let player = Player::new();
        let dealer = Player::new();
        Self {
            deck,
            player,
            dealer,
        }
    }

    pub fn player_turn(&mut self) {
        loop {
            println!("You have {}, dealer has {}.", self.player.get_score(), self.dealer.get_score());
            let action = get_input("What would you like to do?(hit/stay)");
            match action.as_str() {
                "hit" => {
                    self.deck.deal(&mut self.player);
                    if self.player.get_score() > 21 {
                        println!("Dealer wins! You busted!");
                        break;
                    }
                },
                _ => {
                    break;
                }
            }
        }
    }

    pub fn dealer_turn(&mut self) {
        while self.dealer.get_score() < 17 {
            self.deck.deal(&mut self.dealer);
            println!("{:?}", self.dealer.hand.last());
        }
        println!();
    }

    pub fn determine_winner(&mut self) {
        if self.player.get_score() > 21 {
            println!("Dealer wins! You busted! You had {} , dealer had {}", self.player.get_score(), self.dealer.get_score());
        } else if self.dealer.get_score() > 21 {
            println!("You win! Dealer busted! You had {}, dealer had {}", self.player.get_score(), self.dealer.get_score());
        } else if self.player.get_score() == self.dealer.get_score() {
            println!("It's a push! You had {}, dealer had {}", self.player.get_score(), self.dealer.get_score());
        } else if self.player.get_score() > self.dealer.get_score() {
            println!("You win! You had {}, dealer had {}", self.player.get_score(), self.dealer.get_score());
        } else {
            println!("Dealer wins! You had {}, dealer had {}", self.player.get_score(), self.dealer.get_score());
        }
    }
}

pub fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_lowercase()
}