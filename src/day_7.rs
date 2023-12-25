#[path = "utils/file.rs"]
mod file;

use std::cmp::Ordering;
use std::collections::HashMap;
/*
Your all-expenses-paid trip turns out to be a one-way, five-minute ride in an airship. (At least it's a cool airship!) It drops you off at the edge of a vast desert and descends back to Island Island.
"Did you bring the parts?"
You turn around to see an Elf completely covered in white clothing, wearing goggles, and riding a large camel.
"Did you bring the parts?" she asks again, louder this time. You aren't sure what parts she's looking for; you're here to figure out why the sand stopped.
"The parts! For the sand, yes! Come with me; I will show you." She beckons you onto the camel.
After riding a bit across the sands of Desert Island, you can see what look like very large rocks covering half of the horizon.
The Elf explains that the rocks are all along the part of Desert Island that is directly above Island Island, making it hard to even get there.
Normally, they use big machines to move the rocks and filter the sand, but the machines have broken down because Desert Island recently stopped receiving the parts they need to fix the machines.
You've already assumed it'll be your job to figure out why the parts stopped when she asks if you can help. You agree automatically.
Because the journey will take a few days, she offers to teach you the game of Camel Cards. Camel Cards is sort of similar to poker except it's designed to be easier to play while riding a camel.
In Camel Cards, you get a list of hands, and your goal is to order them based on the strength of each hand. A hand consists of five cards labeled one of A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2.
The relative strength of each card follows this order, where A is the highest and 2 is the lowest.
Every hand is exactly one type. From strongest to weakest, they are:
Five of a kind, where all five cards have the same label: AAAAA
Four of a kind, where four cards have the same label and one card has a different label: AA8AA
Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
High card, where all cards' labels are distinct: 23456
Hands are primarily ordered based on type; for example, every full house is stronger than any three of a kind.
If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand.
If these cards are different, the hand with the stronger first card is considered stronger.
If the first card in each hand have the same label, however, then move on to considering the second card in each hand.
If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth.
So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger because its first card is stronger.
Similarly, 77888 and 77788 are both a full house, but 77888 is stronger because its third card is stronger (and both hands have the same first and second card).
To play Camel Cards, you are given a list of hands and their corresponding bid (your puzzle input). For example:
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
This example shows five hands; each hand is followed by its bid amount.
Each hand wins an amount equal to its bid multiplied by its rank, where the weakest hand gets rank 1, the second-weakest hand gets rank 2, and so on up to the strongest hand.
Because there are five hands in this example, the strongest hand will have rank 5 and its bid will be multiplied by 5.
So, the first step is to put the hands in order of strength:
32T3K is the only one pair and the other hands are all a stronger type, so it gets rank 1.
KK677 and KTJJT are both two pair. Their first cards both have the same label, but the second card of KK677 is stronger (K vs T), so KTJJT gets rank 2 and KK677 gets rank 3.
T55J5 and QQQJA are both three of a kind. QQQJA has a stronger first card, so it gets rank 5 and T55J5 gets rank 4.
Now, you can determine the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5).
So the total winnings in this example are 6440.
Find the rank of every hand in your set. What are the total winnings?
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

fn parse_card(ch: char) -> Card {
    match ch {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => panic!("Invalid card label"),
    }
}

fn parse_hand(s: &str) -> Hand {
    let mut iter = s.chars();
    let cards: Vec<Card> = (0..5).map(|_| parse_card(iter.next().unwrap())).collect();
    iter.next();
    let bid: usize = iter.collect::<String>().parse().unwrap();
    Hand { cards, bid }
}

fn count_card_occurrences(cards: &[Card]) -> HashMap<Card, usize> {
    let mut occurrences = HashMap::new();
    for card in cards {
        *occurrences.entry(card.clone()).or_insert(0) += 1;
    }
    occurrences
}

fn evaluate_hand(hand: &Hand) -> (HandType, Vec<Card>) {
    let mut cards = hand.cards.clone();
    let occurrences = count_card_occurrences(&hand.cards);

    // Check for Five of a Kind
    if occurrences.values().any(|&count| count == 5) {
        return (HandType::FiveOfAKind, cards);
    }

    // Check for Four of a Kind
    if occurrences.values().any(|&count| count == 4) {
        return (HandType::FourOfAKind, cards);
    }

    // Check for Full House
    if occurrences.values().any(|&count| count == 3) && occurrences.values().any(|&count| count == 2) {
        return (HandType::FullHouse, cards);
    }

    // Check for Three of a Kind
    if occurrences.values().any(|&count| count == 3) {
        return (HandType::ThreeOfAKind, cards);
    }

    // Check for Two Pair
    if occurrences.values().filter(|&&count| count == 2).count() == 2 {
        return (HandType::TwoPair, cards);
    }

    // Check for One Pair
    if occurrences.values().any(|&count| count == 2) {
        return (HandType::OnePair, cards);
    }

    // High Card
    (HandType::HighCard, cards)
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    let (type1, cards1) = evaluate_hand(hand1);
    let (type2, cards2) = evaluate_hand(hand2);

    if type1 != type2 {
        return type1.cmp(&type2);
    }
    // If hand types are the same, compare individual card values
    for (card1, card2) in cards1.iter().zip(cards2.iter()) {
        let cmp = card1.cmp(card2);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }

    // If all cards are equal, return Equal
    Ordering::Equal
}

fn calculate_total_winnings(mut hands: Vec<Hand>) -> usize {
    let l = hands.len();
    hands.sort_by(|a, b| compare_hands(&a, &b));
    let hands_with_rank: Vec<(Hand, usize)> = hands.into_iter().enumerate().map(|(rank, hand)| (hand, l - rank)).collect();
    //println!("{:?}", hands_with_rank);
    hands_with_rank
        .iter()
        .map(|(hand, rank)| hand.bid * rank)
        .sum()
}


pub fn solve_the_puzzle_7_1() {
    match file::read_file("inputs/day_7_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(input) => {
            let hands: Vec<Hand> = input.lines().map(|s| parse_hand(s)).collect();
            let total_winnings = calculate_total_winnings(hands);
            println!("Total Winnings: {}", total_winnings);
        }
    }
}

pub fn solve_the_puzzle_7_2() {
    match file::read_file("inputs/day_7_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(input) => {

        }
    }
}


