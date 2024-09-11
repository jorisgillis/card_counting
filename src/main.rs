use crate::card::{create_default_stacks, PlayerStacks, Stack};
use crate::game::play_game;
use crate::KingsDecisive::{Drawn, No, Yes};

mod card;
mod game;

fn main() {
    let mut left_wins = 0;
    let mut right_wins = 0;
    let mut draws = 0;
    let mut number_of_rounds: Vec<NumberOfRounds> = vec![];
    let mut kings_decisive_counter = 0;
    let mut kings_drawn_counter = 0;

    let rounds = 10_000;
    for _ in 0..rounds {
        let (game_result, rounds, kings_decisive) = create_stacks_play_game();

        match game_result {
            GameResult::LeftWins => left_wins += 1,
            GameResult::RightWins => right_wins += 1,
            GameResult::Draw => draws += 1,
        }

        if kings_decisive == Yes {
            kings_decisive_counter += 1;
        }
        if kings_decisive == Drawn {
            kings_drawn_counter += 1;
        }

        number_of_rounds.push(rounds);
    }

    let sum_of_rounds: u32 = number_of_rounds.iter().sum();
    println!("Average number of rounds = {}", sum_of_rounds / number_of_rounds.len() as u32);
    println!("Wins for Left  = {}", left_wins);
    println!("Wins for Right = {}", right_wins);
    println!("Draws          = {}", draws);
    println!("Kings Decisive = {}", kings_decisive_counter);
    println!("Kings Drawn    = {}", kings_drawn_counter);
    println!("Kings Beaten   = {}", (rounds - kings_decisive_counter - kings_drawn_counter) as u32);
}

/// The end result of a game: left wins, right wins, or it is a draw.
#[derive(Debug, PartialEq)]
enum GameResult {
    LeftWins,
    RightWins,
    Draw,
}

#[derive(Debug, PartialEq)]
enum KingsDecisive {
    Yes,
    Drawn,
    No,
}

type NumberOfRounds = u32;

/// This function creates stacks for two players and plays one game.
///
/// After the game is played, the game result and the number of rounds that has been played is
/// returned.
fn create_stacks_play_game() -> (GameResult, NumberOfRounds, KingsDecisive) {
    let (mut left_player, mut right_player) = create_default_stacks();

    let left_kings = count_kings(left_player.draw_stack());
    let right_kings = count_kings(right_player.draw_stack());

    let number_of_rounds = play_game(&mut left_player, &mut right_player);

    let result = game_outcome(&mut left_player, &mut right_player);

    let kings_desicive = match result {
        GameResult::LeftWins => {
            if left_kings > right_kings {
                Yes
            } else if left_kings == right_kings {
                Drawn
            } else {
                No
            }
        }
        GameResult::RightWins => {
            if left_kings < right_kings {
                Yes
            } else if left_kings == right_kings {
                Drawn
            } else {
                No
            }
        }
        GameResult::Draw => {
            No
        }
    };

    (result, number_of_rounds, kings_desicive)
}

fn count_kings(stack: &Stack) -> u8 {
    let mut king_counter: u8 = 0;
    for card in stack {
        if card.is_king() {
            king_counter += 1;
        }
    }

    king_counter
}

/// Decides on the GameResult based on the number of cards each player has.
///
/// The player that doesn't have any cards left has lost. In any other constellation, the game has
/// been ended because the maximum number of rounds has been exceeded.
fn game_outcome(left_player: &PlayerStacks, right_player: &PlayerStacks) -> GameResult {
    match (left_player.total_cards(), right_player.total_cards()) {
        (0, 0) => GameResult::Draw,
        (0, _) => GameResult::RightWins,
        (_, 0) => GameResult::LeftWins,
        _ => GameResult::Draw
    }
}

#[cfg(test)]
mod test {
    use crate::card::{Card, PlayerStacks, Suite};
    use crate::{game_outcome, GameResult};
    use std::collections::VecDeque;

    #[test]
    fn test_game_outcome_both_have_no_cards() {
        let left_player = PlayerStacks::new(VecDeque::new());
        let right_player = PlayerStacks::new(VecDeque::new());

        let result = game_outcome(&left_player, &right_player);

        assert_eq!(result, GameResult::Draw);
    }

    #[test]
    fn test_game_outcome_left_wins() {
        let left_player = PlayerStacks::new(
            VecDeque::from([
                Card::create(Suite::Club, 1).unwrap()
            ])
        );
        let right_player = PlayerStacks::new(VecDeque::from([]));

        let result = game_outcome(&left_player, &right_player);

        assert_eq!(result, GameResult::LeftWins);
    }

    #[test]
    fn test_game_outcome_right_wins() {
        let right_player = PlayerStacks::new(
            VecDeque::from([
                Card::create(Suite::Club, 1).unwrap()
            ])
        );
        let left_player = PlayerStacks::new(VecDeque::from([]));

        let result = game_outcome(&left_player, &right_player);

        assert_eq!(result, GameResult::RightWins);
    }

    #[test]
    fn test_game_aborted_means_draw() {
        let left_player = PlayerStacks::new(
            VecDeque::from([
                Card::create(Suite::Heart, 12).unwrap()
            ])
        );
        let right_player = PlayerStacks::new(
            VecDeque::from([
                Card::create(Suite::Club, 1).unwrap()
            ])
        );

        let result = game_outcome(&left_player, &right_player);

        assert_eq!(result, GameResult::Draw);
    }
}
