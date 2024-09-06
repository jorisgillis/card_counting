use enum_iterator::{all, Sequence};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::VecDeque;

const DECK_SIZE: usize = 52;
const SUITE_SIZE: usize = 13;

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

pub fn create_stacks() -> (VecDeque<Card>, VecDeque<Card>) {
    let mut deck = deck();
    let deck_size = deck.len();
    if deck_size % 2 != 0 {
        panic!("Deck should be divisible by two");
    }

    shuffle_deck(&mut deck);

    (VecDeque::from(deck[0..deck_size / 2].to_vec()), VecDeque::from(deck[(deck_size / 2)..deck_size].to_vec()))
}
fn deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];
    for suite in all::<Suite>() {
        for number in 1..(SUITE_SIZE + 1) {
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
    use crate::card::{create_stacks, deck, shuffle_deck, Card, Suite, DECK_SIZE, SUITE_SIZE};
    use itertools::Itertools;
    use std::collections::VecDeque;

    #[test]
    fn test_create_deck() {
        let deck = deck();

        assert_eq!(deck.len(), DECK_SIZE);
        let suites: Vec<Suite> = deck.iter().map(|card| card.suite).collect();
        let numbers: Vec<u8> = deck.iter().map(|card| card.number).collect();
        assert_eq!(suites.iter().unique().count(), 4);
        assert_eq!(numbers.iter().unique().count(), SUITE_SIZE);
    }

    #[test]
    fn test_ordered_deck() {
        let deck = deck();
        assert!(is_ordered_deck(&mut VecDeque::from(deck)))
    }

    #[test]
    fn test_shuffled_deck_is_not_ordered() {
        let mut deck = deck();
        shuffle_deck(&mut deck);

        assert_eq!(deck.len(), DECK_SIZE);
        assert!(!is_ordered_deck(&mut VecDeque::from(deck)));
    }

    #[test]
    fn test_stacks_are_shuffled() {
        let (mut stack1, mut stack2) = create_stacks();

        assert_eq!(stack1.len(), stack2.len());
        assert!(!is_ordered_deck(&mut stack1));
        assert!(!is_ordered_deck(&mut stack2));
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