use crate::card::create_default_stacks;
use crate::game::play_game;

mod card;
mod game;

fn main() {
    let mut left_wins = 0;
    let mut right_wins = 0;
    let mut draws = 0;
    let mut number_of_rounds: Vec<NumberOfRounds> = vec![];

    for _ in 0..10000 {
        let (game_result, rounds) = create_stacks_play_game();

        match game_result {
            GameResult::LeftWins => left_wins += 1,
            GameResult::RightWins => right_wins += 1,
            GameResult::Draw => draws += 1,
        }

        number_of_rounds.push(rounds);
    }

    let sum_of_rounds: u32 = number_of_rounds.iter().sum();
    println!("Average number of rounds = {}", sum_of_rounds / number_of_rounds.len() as u32);
    println!("Wins for Left  = {}", left_wins);
    println!("Wins for Right = {}", right_wins);
    println!("Draws          = {}", draws);
}

enum GameResult {
    LeftWins,
    RightWins,
    Draw,
}

type NumberOfRounds = u32;

fn create_stacks_play_game() -> (GameResult, NumberOfRounds) {
    let (mut left_player, mut right_player) = create_default_stacks();

    let number_of_rounds = play_game(&mut left_player, &mut right_player);

    let result = match (left_player.total_cards(), right_player.total_cards()) {
        (0, _) => GameResult::RightWins,
        (_, 0) => GameResult::LeftWins,
        _ => GameResult::Draw
    };

    (result, number_of_rounds)
}
