use blackjack::*;

fn main() {
    //Start game loop
    println!("Welcome to BlackJack!");
    let mut game_loop = true;
    while game_loop == true {
        //Creates new deck
        let mut deck = Deck::new();
        //Shuffles deck
        deck.shuffle();

        let mut player = Player::new();
        let mut dealer = Player::new();
        //Deals player and Dealer cards
        deck.deal(&mut player);
        deck.deal(&mut dealer);
        deck.deal(&mut player);

        //Start hand loop
        let mut hand_loop = true;
        while hand_loop == true {
            println!("You have {}, dealer has {}.", player.get_score(), dealer.get_score());
            println!("What would you like to do? (hit/stay)", );
            let mut play = String::new();
            std::io::stdin().read_line(&mut play).expect("Failed to read line");
            match play.trim() {
                "hit" => {
                    deck.deal(&mut player);
                    if player.get_score() > 21 {
                        println!("You busted! Dealer wins!");
                        println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                        hand_loop = false;
                    }},
                _ => { 
                    while dealer.get_score() < 17 {
                        deck.deal(&mut dealer);
                    }
                    if dealer.get_score() > 21 {
                        println!("Dealer busted, you win!");
                        println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                        hand_loop = false;
                    } else if dealer.get_score() == 21 {
                        println!("It's a push!");
                        println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                        hand_loop = false;
                    } else {
                        if dealer.get_score() > player.get_score() {
                            println!("Dealer wins! Dealer had:");
                            println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                            hand_loop = false;
                        } else {
                            println!("You win!");
                            println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                            hand_loop = false;
                        }
                    }
                }
            }
            
        }
        println!();
        println!("Would you like to play again? (y/n)");
        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again).expect("Failed to read line");
        if play_again.trim().to_lowercase() == "y" {
            game_loop = true;
            println!();
        } else {
            game_loop = false;
        }
    }
}