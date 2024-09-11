# Card Counter

This small application plays a game of "highest card wins" where ties are broken by drawing another round of cards.
The goal is twofold:

1. Learning to make a small application in Rust.
2. Get insights into the properties of these games:
    1. How many rounds are played on average?
    2. Is the number of 13s deceive for the game result?

## The game

This is a very simple two-player game.
A deck of cards is shuffled and each player gets half of the cards.
Hence, a player has a random half of the deck.
Next, the players play round after round until one player has no cards left.

A round consists of each player drawing a card from their draw stack.
The player with the highest card (based on rank, suite is irrelevant) wins both cards and places them on the won stack.
If there is a draw, both players draw another card from the draw stack until one player has a higher card.

If the draw stack is empty, the stack of won cards is shuffled and becomes the draw stack.

## The results

### How many rounds?

Without any statistical analysis, it seems that the average number of rounds of games tends to be around
560 to 580.

### Is the number of 13s decisive for the outcome of the game?

The number 13 is also known as the king in a deck of cards.
This is the highest card in this game.
A king can only be taken by another player when two 13's tie and the other player wins the tie.

In about 60% of the cases there is a majority of kings.
Of those cases, the player with the majority wins about 80% of the time, still loosing about 20% of the games.
In the other 40% of cases, the number of kings was equally distributed between both players.

We can conclude that having the majority of kings is a good indication of you winning.

## How to run?

Given that you have a Rust installation up and running, the easiest way to run the program is by using cargo:

```
cargo run
```

## Caveat

Do note that this program is an experiment whilst learning to code in Rust.
Probably there are many things that can be optimized or should be done differently.
My main interest was to be able to write some Rust code that go beyond single file programs, to get some insights
into the ways a Rust program can be structured. 
