use enum_iterator::{all, Sequence};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;


const SUITE_SIZE: usize = 13;

/// A Card is a combination of a suite (the symbol on the card) and its rank within that suite.
///
/// A Card represents a card from a deck of playing cards. Typically, there are four suites
/// (hearts, diamonds, spades and clubs). The cards are ranked 1 to 10 with three additional cards
/// for the jack (11), queen (12) and king (13).
///
/// There is no compare function on the cards, as different games have different comparison rules.
///
/// To create a new card:
/// ```
/// Card::new(Suite::Heart, 1)
/// ```
#[derive(Clone, Debug)]
pub struct Card {
    pub suite: Suite,
    pub number: u8,
}

impl Card {
    pub fn create(suite: Suite, number: u8) -> Option<Card> {
        if number < 1 || number > SUITE_SIZE as u8 {
            None
        } else {
            Some(
                Card {
                    suite,
                    number,
                }
            )
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Sequence, Copy, Clone)]
pub enum Suite {
    Heart,
    Club,
    Diamond,
    Spade,
}

/// A Stack is a sequence of cards.
pub type Stack = VecDeque<Card>;

/// A PlayerStacks is the combination of a draw stack (still to be played) and won stack
/// (won during the previous rounds).
///
/// The draw stack is the stack of cards that the player can still play. If the draw stack is empty
/// the won stack is shuffled and re-purposed as the draw stack.
/// On the won stack all the cards are deposited that the player has won.
///
/// ```
/// let player_stacks = PlayerStacks::new(deck());
///
/// if player_stacks.is_empty() {
///     panic("Shouldn't be empty!")
/// }
///
/// let initial_number_of_cards = player_stacks.total_cards();
///
/// // To get the first card of the draw stack (or the reshuffled won stack if the draw stack is empty)
/// let first_card: Option<Card> = player_stacks.pop_front();
///
/// assert_eq!(initial_number_of_cards - 1, player_stacks.total_cards());
/// ```
#[derive(Debug)]
pub struct PlayerStacks {
    draw_stack: Stack,
    won_stack: Stack,
}

impl PlayerStacks {
    /// Create a new pair of stacks based on a deck of cards and an empty won stack.
    pub fn new(stack: VecDeque<Card>) -> PlayerStacks {
        PlayerStacks {
            draw_stack: stack,
            won_stack: VecDeque::new(),
        }
    }

    /// A reference to the draw stack of this player.
    pub fn draw_stack(&self) -> &Stack {
        &self.draw_stack
    }

    /// A reference to the won stack of this player.
    pub fn won_stack(&self) -> &Stack {
        &self.won_stack
    }

    /// Checks whether both stacks are empty.
    pub fn is_empty(&self) -> bool {
        self.draw_stack.is_empty() && self.won_stack.is_empty()
    }

    /// Retrieves (and removes) the first card of the draw stack, reshuffling the won stack if
    /// needed.
    pub fn pop_front(&mut self) -> Option<Card> {
        if self.draw_stack.is_empty() && !self.won_stack.is_empty() {
            self.shuffle_won_stack()
        }
        self.draw_stack.pop_front()
    }

    /// Appends a list of cards to the won stack.
    pub fn append(&mut self, cards: &mut VecDeque<Card>) {
        self.won_stack.append(cards);
    }

    /// The total number of cards in both the draw and won stacks.
    pub fn total_cards(&self) -> usize {
        self.draw_stack.len() + self.won_stack.len()
    }

    fn shuffle_won_stack(&mut self) {
        let mut rng = thread_rng();
        self.won_stack.make_contiguous().shuffle(&mut rng);
        self.draw_stack = VecDeque::new();
        self.draw_stack.append(&mut self.won_stack);
        self.won_stack = VecDeque::new();
    }
}

/// Creates two PlayerStacks with default settings (4 suites; 13 cards per suite).
///
/// The default deck of cards has 4 suites each with 13 cards. This function creates a default deck
/// of cards, shuffles them and puts them into to equal draw stacks (one for each player). The won
/// stacks of both players is empty.
pub fn create_default_stacks() -> (PlayerStacks, PlayerStacks) {
    create_stacks(SUITE_SIZE)
}

/// Creates two non-default PlayerStacks with 4 suites and the requested number of cards.
///
/// This function is mainly meant for debugging purposes.
pub fn create_stacks(suite_size: usize) -> (PlayerStacks, PlayerStacks) {
    let mut deck = deck(suite_size);
    let deck_size = deck.len();
    if deck_size % 2 != 0 {
        panic!("Deck should be divisible by two");
    }

    shuffle_deck(&mut deck);

    (
        PlayerStacks {
            draw_stack: VecDeque::from(deck[0..deck_size / 2].to_vec()),
            won_stack: VecDeque::new(),
        },
        PlayerStacks {
            draw_stack: VecDeque::from(deck[(deck_size / 2)..deck_size].to_vec()),
            won_stack: VecDeque::new(),
        }
    )
}

fn deck(suite_size: usize) -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    for suite in all::<Suite>() {
        for number in 1..(suite_size + 1) {
            deck.push(Card {
                suite,
                number: u8::try_from(number).unwrap(),
            })
        }
    }

    deck
}

fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
}


#[cfg(test)]
mod test {
    use crate::card::{create_default_stacks, deck, shuffle_deck, Card, Suite, SUITE_SIZE};
    use itertools::Itertools;
    use std::collections::VecDeque;

    const DECK_SIZE: usize = 52;

    #[test]
    fn test_create_deck() {
        let deck = deck(SUITE_SIZE);

        assert_eq!(deck.len(), DECK_SIZE);
        let suites: Vec<Suite> = deck.iter().map(|card| card.suite).collect();
        let numbers: Vec<u8> = deck.iter().map(|card| card.number).collect();
        assert_eq!(suites.iter().unique().count(), 4);
        assert_eq!(numbers.iter().unique().count(), SUITE_SIZE);
    }

    #[test]
    fn test_ordered_deck() {
        let deck = deck(SUITE_SIZE);
        assert!(is_ordered_deck(&mut VecDeque::from(deck)))
    }

    #[test]
    fn test_shuffled_deck_is_not_ordered() {
        let mut deck = deck(SUITE_SIZE);
        shuffle_deck(&mut deck);

        assert_eq!(deck.len(), DECK_SIZE);
        assert!(!is_ordered_deck(&mut VecDeque::from(deck)));
    }

    #[test]
    fn test_stacks_are_shuffled() {
        let (mut left_player, mut right_player) = create_default_stacks();

        assert_eq!(left_player.draw_stack.len(), right_player.draw_stack.len());
        assert!(!is_ordered_deck(&mut left_player.draw_stack));
        assert!(!is_ordered_deck(&mut right_player.draw_stack));
    }

    fn is_ordered_deck(deck: &mut VecDeque<Card>) -> bool {
        let mut previous_card = deck.front().unwrap();
        let mut ordered = true;

        for card in deck.iter().skip(1) {
            if card.suite != previous_card.suite {
                if card.number != 1 {
                    ordered = false;
                    break;
                }
            } else {
                if card.number <= previous_card.number {
                    ordered = false;
                    break;
                }
            }
            previous_card = card;
        }

        ordered
    }
}