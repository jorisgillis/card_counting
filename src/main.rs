use crate::card::create_stacks;
use crate::game::play_game;

mod card;
mod game;

fn main() {
    let (mut stack1, mut stack2) = create_stacks();

    println!("{:?}", stack1);
    println!("{:?}", stack2);

    let number_of_rounds = play_game(&mut stack1, &mut stack2);

    println!("Number of rounds = {:?}", number_of_rounds);
    println!("{:?}", stack1);
    println!("{:?}", stack2);
}
