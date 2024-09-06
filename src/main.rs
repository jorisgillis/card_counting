use crate::card::create_default_stacks;
use crate::game::play_game;

mod card;
mod game;

fn main() {
    let (mut left_player, mut right_player) = create_default_stacks();
    
    let number_of_rounds = play_game(&mut left_player, &mut right_player);

    println!("Number of rounds = {:?}", number_of_rounds);
    println!("{:?}", left_player);
    println!("{:?}", right_player);
}
