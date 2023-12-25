#[path = "utils/file.rs"]
mod file;

use std::cmp::Ordering;
use std::collections::HashMap;
/*
--- Part Two ---
To make things a little more interesting, the Elf introduces one additional rule. Now, J cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.
To balance this, J cards are now the weakest individual cards, weaker even than 2. The other cards stay in the same order: A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J.
J cards can pretend to be whatever card is best for the purpose of determining hand type; for example, QJJQ2 is now considered four of a kind.
However, for the purpose of breaking ties between two hands of the same type, J is always treated as J, not the card it's pretending to be: JKKK2 is weaker than QQQQ2 because J is weaker than Q.
Now, the above example goes very differently:
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
32T3K is still the only one pair; it doesn't contain any jokers, so its strength doesn't increase.
KK677 is now the only two pair, making it the second-weakest hand.
T55J5, KTJJT, and QQQJA are now all four of a kind! T55J5 gets rank 3, QQQJA gets rank 4, and KTJJT gets rank 5.
With the new joker rule, the total winnings in this example are 5905.
Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
*/
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
enum Card {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
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
    let mut total_card_j = 0;
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
        match occurrences.get(&Card::J) {
            Some(&value) => {
                return (HandType::FiveOfAKind, cards);
            }
            None => {
                return (HandType::FourOfAKind, cards);
            }
        }
    }

    // Check for Full House
    if occurrences.values().any(|&count| count == 3) && occurrences.values().any(|&count| count == 2) {
        match occurrences.get(&Card::J) {
            Some(&value) => {
                match value {
                    2 => {
                        return (HandType::FiveOfAKind, cards);
                    }
                    3 => {
                        return (HandType::FiveOfAKind, cards);
                    }
                    _ => {
                        return (HandType::FullHouse, cards);
                    }
                }
            }
            None => {
                return (HandType::FullHouse, cards);
            }
        }
    }

    // Check for Three of a Kind
    if occurrences.values().any(|&count| count == 3) {
        match occurrences.get(&Card::J) {
            Some(&value) => {
                if value == 1 || value == 3{
                    return (HandType::FourOfAKind, cards);
                } else {
                    return (HandType::ThreeOfAKind, cards);
                }
            }
            None => {
                return (HandType::ThreeOfAKind, cards);
            }
        }
    }

    // Check for Two Pair
    if occurrences.values().filter(|&&count| count == 2).count() == 2 {
        match occurrences.get(&Card::J) {
            Some(&value) => {
                match value {
                    1 => {
                        return (HandType::FullHouse, cards);
                    }
                    2 => {
                        return (HandType::FourOfAKind, cards);
                    }
                    _ => {
                        return (HandType::TwoPair, cards);
                    }
                }
            }
            None => {
                return (HandType::TwoPair, cards);
            }
        }
    }

    // Check for One Pair
    if occurrences.values().any(|&count| count == 2) {
        match occurrences.get(&Card::J) {
            Some(&value) => {
                if value == 1 || value == 2{
                    return (HandType::ThreeOfAKind, cards);
                } else {
                    return (HandType::OnePair, cards);
                }
            }
            None => {
                return (HandType::OnePair, cards);
            }
        }
    }

    match occurrences.get(&Card::J) {
        Some(&value) => {
            if value == 1 {
                return (HandType::OnePair, cards);
            } else {
                return (HandType::HighCard, cards);
            }
        }
        None => {
            return (HandType::HighCard, cards);
        }
    }
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


pub fn solve_the_puzzle_7_2() {
    match file::read_file("inputs/day_7_1.txt") {
        Err(error) => {
            println!("error {}", error)
        }
        Ok(input) => {
            let hands: Vec<Hand> = input.lines().map(|s| parse_hand(s)).collect();
            let total_winnings = calculate_total_winnings(hands);
            println!("Total Winnings Part 2: {}", total_winnings);
        }
    }
}


