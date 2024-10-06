use blackjack::*; // Import the necessary components from the blackjack module



fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_lowercase()
}

struct GameState {
    deck: Deck,
    player: Player,
    dealer: Player,
}

impl GameState {
    fn new() -> Self {
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

    fn player_turn(&mut self) {
        while true {
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

    fn dealer_turn(&mut self) {
        while self.dealer.get_score() < 17 {
            self.deck.deal(&mut self.dealer);
            println!("{:?}", self.dealer.hand.last());
        }
        println!();
    }

    fn determine_winner(&mut self) {
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

fn main() {
    println!("Welcome to BlackJack!");

    loop {
        let mut game_state = GameState::new();
        game_state.deck.deal(&mut game_state.player);
        game_state.deck.deal(&mut game_state.dealer);
        game_state.deck.deal(&mut game_state.player);
        game_state.player_turn();
        game_state.dealer_turn();
        game_state.determine_winner();

        if get_input("Would you like to play again?") != "y" {
            break;
        }
    }
}