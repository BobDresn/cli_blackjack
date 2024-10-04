use blackjack::*; // Import the necessary components from the blackjack module

fn main() {
    // Start game loop
    println!("Welcome to BlackJack!"); // Welcome message
    let mut game_loop = true; // Control variable for the game loop
    
    while game_loop == true { // Main game loop
        // Creates a new deck of cards
        let mut deck = Deck::new();
        // Shuffles the deck
        deck.shuffle();

        // Creates new players for the game
        let mut player = Player::new();
        let mut dealer = Player::new();
        
        // Deals two cards to the player and one card to the dealer
        deck.deal(&mut player); // Deal first card to player
        deck.deal(&mut dealer); // Deal card to dealer
        deck.deal(&mut player); // Deal second card to player

        // Start hand loop
        let mut hand_loop = true; // Control variable for the hand loop
        
        while hand_loop == true { // Loop for the current hand
            // Display current scores for player and dealer
            println!("You have {}, dealer has {}.", player.get_score(), dealer.get_score());
            println!("What would you like to do? (hit/stay)");
            
            // Read player input for their action
            let mut play = String::new();
            std::io::stdin().read_line(&mut play).expect("Failed to read line");
            
            match play.trim() { // Match on the player's input
                "hit" => {
                    // Deal another card to the player
                    deck.deal(&mut player);
                    // Check if the player has busted (exceeded 21 points)
                    if player.get_score() > 21 {
                        println!("You busted! Dealer wins!"); // Inform the player they busted
                        println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                        hand_loop = false; // Exit the hand loop
                    }
                },
                _ => { // If the player chooses to stay
                    // Dealer hits until they reach at least 17
                    while dealer.get_score() < 17 {
                        deck.deal(&mut dealer);
                    }
                    
                    // Check if the dealer has busted
                    if dealer.get_score() > 21 {
                        println!("Dealer busted, you win!"); // Inform the player of their win
                        println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                        hand_loop = false; // Exit the hand loop
                    } else if dealer.get_score() == 21 {
                        println!("It's a push!"); // Inform the player of a tie
                        println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                        hand_loop = false; // Exit the hand loop
                    } else {
                        // Compare scores to determine the winner
                        if dealer.get_score() > player.get_score() {
                            println!("Dealer wins! Dealer had:"); // Dealer wins
                            println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                            hand_loop = false; // Exit the hand loop
                        } else {
                            println!("You win!"); // Player wins
                            println!("You had {}, dealer had {}.", player.get_score(), dealer.get_score());
                            hand_loop = false; // Exit the hand loop
                        }
                    }
                }
            }
        }
        
        // Ask if the player wants to play again
        println!();
        println!("Would you like to play again? (y/n)");
        let mut play_again = String::new();
        std::io::stdin().read_line(&mut play_again).expect("Failed to read line");
        
        // Check player response for continuing the game
        if play_again.trim().to_lowercase() == "y" {
            game_loop = true; // Restart the game loop
            println!(); // Add a newline for better readability
        } else {
            game_loop = false; // End the game loop
        }
    }
}