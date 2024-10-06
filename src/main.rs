fn main() {
    println!("Welcome to BlackJack!");

    loop {
        //Creates new deck, player, and dealer
        let mut game_state = blackjack::GameState::new();

        //Initial hands are dealt
        game_state.deck.deal(&mut game_state.player);
        game_state.deck.deal(&mut game_state.dealer);
        game_state.deck.deal(&mut game_state.player);

        //Player can hit/stay until done
        game_state.player_turn();

        //Dealer will hit until 17
        //Doesn't hit on soft 17
        game_state.dealer_turn();

        //Decides game winner
        game_state.determine_winner();

        //Game ended
        if blackjack::get_input("Would you like to play again?") != "y" {
            break;
        }
    }
}