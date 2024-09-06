use crate::card::Card;
use crate::card::PlayerStacks;
use crate::game::RoundResult::{Continue, GameOver};
use std::collections::VecDeque;

/// Play a game with two stacks of cards. This is a higher-wins game, meaning the top cards
/// are compared and the higher one wins both cards. In case of a tie, another card is drawn to
/// break the tie. If there are no more cards available to break a tie, the stack that has the
/// remaining cards wins.
pub fn play_game(left_player: &mut PlayerStacks, right_player: &mut PlayerStacks) -> u32 {
    let mut round_counter = 0;
    let mut game_state: RoundResult = Continue;

    if left_player.is_empty() || right_player.is_empty() {
        game_state = GameOver;
    }

    while game_state == Continue {
        round_counter += 1;
        game_state = play_round(left_player, right_player);

        if round_counter > 100_000 {
            // Safety check, let's not go too far!
            break;
        }
    }

    round_counter
}

#[derive(PartialEq, Debug)]
enum RoundResult {
    Continue,
    GameOver,
}


enum ResolutionResult {
    LeftWins,
    RightWins,
    GameOver,
}

/// Play one round of highest-wins: get card(s) from the top of both stacks and place them back
/// in the winning deck.
/// Ties are broken by drawing more cards (or if a stack runs out of cards).
fn play_round(left: &mut PlayerStacks, right: &mut PlayerStacks) -> RoundResult {
    let (mut won_cards, winner) = resolve_round(left, right);

    match winner {
        ResolutionResult::LeftWins => {
            left.append(&mut won_cards);
            Continue
        }
        ResolutionResult::RightWins => {
            right.append(&mut won_cards);
            Continue
        }
        ResolutionResult::GameOver => {
            GameOver
        }
    }
}

fn resolve_round(left: &mut PlayerStacks, right: &mut PlayerStacks) -> (VecDeque<Card>, ResolutionResult) {
    if left.is_empty() || right.is_empty() {
        let deque: VecDeque<Card> = VecDeque::new();
        return (deque, ResolutionResult::GameOver);
    }

    let mut collected_cards: VecDeque<Card> = VecDeque::new();

    let left_card = left.pop_front().unwrap();
    let right_card = right.pop_front().unwrap();

    let comparison_result = compare_cards(&left_card, &right_card);

    collected_cards.push_back(left_card);
    collected_cards.push_back(right_card);

    match comparison_result {
        CompareResult::LeftWins => {
            (collected_cards, ResolutionResult::LeftWins)
        }
        CompareResult::RightWins => {
            (collected_cards, ResolutionResult::RightWins)
        }
        CompareResult::Equal => {
            let (mut resolved_cards, round_result) = resolve_round(left, right);

            collected_cards.append(&mut resolved_cards);

            (collected_cards, round_result)
        }
    }
}


enum CompareResult {
    Equal,
    LeftWins,
    RightWins,
}

fn compare_cards(left_card: &Card, right_card: &Card) -> CompareResult {
    if left_card.number == right_card.number {
        CompareResult::Equal
    } else if left_card.number > right_card.number {
        CompareResult::LeftWins
    } else {
        CompareResult::RightWins
    }
}

#[cfg(test)]
mod test {
    use crate::card::{Card, PlayerStacks, Suite};
    use crate::game::{play_game, play_round, RoundResult};
    use std::collections::VecDeque;

    #[test]
    fn test_first_stack_wins_round() {
        let mut stack1 = PlayerStacks::new(VecDeque::from([Card::create(Suite::Club, 13).unwrap()]));
        let mut stack2 = PlayerStacks::new(VecDeque::from([Card::create(Suite::Club, 1).unwrap()]));

        play_round(&mut stack1, &mut stack2);

        assert_eq!(stack1.won_stack().len(), 2);
        assert_eq!(stack2.won_stack().len(), 0);
    }

    #[test]
    fn test_second_stack_wins_round() {
        let mut stack1 = PlayerStacks::new(VecDeque::from([Card::create(Suite::Club, 1).unwrap()]));
        let mut stack2 = PlayerStacks::new(VecDeque::from([Card::create(Suite::Club, 2).unwrap()]));

        play_round(&mut stack1, &mut stack2);

        assert_eq!(stack1.won_stack().len(), 0);
        assert_eq!(stack2.won_stack().len(), 2);
    }

    #[test]
    fn test_tie_broken_by_next_card() {
        let mut stack1 = PlayerStacks::new(VecDeque::from([
            Card::create(Suite::Club, 1).unwrap(),
            Card::create(Suite::Club, 13).unwrap(),
            Card::create(Suite::Club, 10).unwrap(),
        ]));
        let mut stack2 = PlayerStacks::new(VecDeque::from([
            Card::create(Suite::Club, 1).unwrap(),
            Card::create(Suite::Club, 12).unwrap(),
            Card::create(Suite::Club, 11).unwrap(),
            Card::create(Suite::Club, 12).unwrap(),
        ]));

        play_round(&mut stack1, &mut stack2);

        assert_eq!(stack1.total_cards(), 5);
        assert_eq!(stack2.total_cards(), 2);
    }

    #[test]
    fn test_complete_tie_results_in_gameover_with_empty_stacks() {
        let mut stack1 = PlayerStacks::new(create_deck());
        let mut stack2 = PlayerStacks::new(create_deck());

        let result = play_round(&mut stack1, &mut stack2);

        assert_eq!(result, RoundResult::GameOver);
        assert_eq!(stack1.total_cards(), 0);
        assert_eq!(stack2.total_cards(), 0);
    }

    fn create_deck() -> VecDeque<Card> {
        VecDeque::from([
            Card::create(Suite::Club, 1).unwrap(),
            Card::create(Suite::Club, 13).unwrap(),
            Card::create(Suite::Club, 10).unwrap(),
        ])
    }

    #[test]
    fn test_left_has_a_winning_game() {
        let mut stack1 = PlayerStacks::new(VecDeque::from([
            Card::create(Suite::Club, 13).unwrap(),
            Card::create(Suite::Club, 10).unwrap(),
            Card::create(Suite::Heart, 8).unwrap(),
        ]));
        let mut stack2 = PlayerStacks::new(VecDeque::from([
            Card::create(Suite::Spade, 10).unwrap(),
            Card::create(Suite::Diamond, 10).unwrap(),
            Card::create(Suite::Diamond, 9).unwrap(),
        ]));

        let rounds = play_game(&mut stack1, &mut stack2);

        assert!(!stack1.is_empty());
        assert!(stack2.is_empty());
        assert!(rounds > 2);
    }
}