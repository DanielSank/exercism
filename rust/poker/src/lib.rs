/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::collections::{HashSet, HashMap};

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut h = vec![];  // Hand, &str
    for hand_str in hands {
        h.push((best_hand(hand_str), hand_str));
    }
    h.sort();
    let mut winning_hand_strs = vec![];
    let (winning_hand, winning_hand_str) = h.pop().unwrap();
    winning_hand_strs.push(*winning_hand_str);
    for (hand, hand_str) in h {
        if hand == winning_hand {
            winning_hand_strs.push(hand_str)
        }
    }
    winning_hand_strs
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Hand {
    HighCard(usize, usize, usize, usize, usize),  // Card ranks in descending order
    OnePair(usize, usize, usize, usize),   // Rank of pair, then ranks of high cards
    TwoPair(usize, usize, usize),  // Ranks of the pairs in descending order, then rank of remaining card
    ThreeOfAKind(usize, usize, usize),  // Rank of the three-of-a-kind, then rank of remaining cards in descending order
    Straight(usize),  // Highest card in the straight
    Flush(usize, usize, usize, usize, usize),  // Ranks of cards in descending order
    FullHouse(usize, usize),  // Rank of three-of-a-kind, rank of pair
    FourOfAKind(usize, usize),  // Rank of the four, then rank of remaining card
    StraightFlush(usize),  // Highest cast in the straight-flush
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Heart,
    Club,
    Diamond,
    Spade,
}

impl Suit {
    pub fn new(s: &str) -> Suit {
        use Suit::*;
        match s {
            "H" => Heart,
            "C" => Club,
            "D" => Diamond,
            "S" => Spade,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    pub rank: usize,
    pub suit: Suit,
}

impl Card {
    pub fn new(s: &str) -> Card {
        let (rank_str, suit_str) = s.split_at(s.len() - 1);
        Card {
            rank: match rank_str {
                "J" => 11,
                "Q" => 12,
                "K" => 13,
                "A" => 14,
                c => str::parse::<usize>(c).unwrap() as usize,
            },
            suit: Suit::new(suit_str),
        }
    }
    pub fn from_str(a: &str) -> Vec<Card> {
        let mut v = vec![];
        for s in a.split(" ") {
            v.push(Card::new(s));
        }
        v
    }
}

// rank -> ocurrence
fn count_ranks(cards: &Vec<Card>) -> HashMap<usize, usize> {
    let mut ranks = HashMap::new();
    for card in cards {
        let val = ranks.entry(card.rank).or_insert(0);
        *val += 1;
    }
    ranks
}

pub fn best_paired_hand(cards: &Vec<Card>) -> Hand {
    use Hand::*;
    let ranks = count_ranks(&cards);
    let mut ranks_with_one = vec![];
    let mut ranks_with_pairs = vec![];
    let mut rank_with_three: Option<usize> = None;
    let mut rank_with_four: Option<usize> = None;
    for (rank, ocurrence) in ranks.iter() {
        match ocurrence {
            4 => rank_with_four = Some(*rank),
            3 => rank_with_three = Some(*rank),
            2 => ranks_with_pairs.push(*rank),
            1 => ranks_with_one.push(*rank),
            _ => panic!("Found {} cards of rank {}. Are you cheating?", ocurrence, rank)
        };
    }
    ranks_with_pairs.sort();
    ranks_with_one.sort();
    match rank_with_four {
        None => (),
        Some(r) => return FourOfAKind(r, ranks_with_one[0]),
    }
    match rank_with_three {
        None => (),
        Some(r) => match ranks_with_pairs.len() {
            1 => return FullHouse(r, ranks_with_pairs[0]),
            0 => return ThreeOfAKind(r, ranks_with_one[1], ranks_with_one[0]),
            _ => panic!(),  // unreachable
        },
    }
    match ranks_with_pairs.len() {
        2 => return TwoPair(ranks_with_pairs[1], ranks_with_pairs[0], ranks_with_one[0]),
        1 => return OnePair(ranks_with_pairs[0], ranks_with_one[2], ranks_with_one[1], ranks_with_one[0]),
        _ => (),
    }
    HighCard(ranks_with_one[4], ranks_with_one[3], ranks_with_one[2], ranks_with_one[1], ranks_with_one[0])
}

pub fn suited_and_or_straight_hand(cards: &Vec<Card>) -> Option<Hand> {
    use Hand::*;
    let suits: HashSet<Suit> = HashSet::from_iter(cards.iter().map(|x| x.suit));
    let mut ranks = Vec::from_iter(cards.iter().map(|x| x.rank));
    ranks.sort();

    // A straight can happen with all ranks in sequential order...
    let mut have_straight_aces_high = true;
    for idx in 1..ranks.len() {
        if ranks[idx] != ranks[idx - 1] + 1 {
            have_straight_aces_high = false;
        }
    }
    // ...unless the low card is ace (rank=14) in which case ranks = [2, 3, 4, 5, 14]
    let have_straight_aces_low = ranks == vec![2, 3, 4, 5, 14];
    let have_straight = have_straight_aces_low || have_straight_aces_high;

    let straight_high_card = match have_straight {
        false => None,
        true => match have_straight_aces_high {
            true => Some(ranks[ranks.len() - 1]),
            false => Some(ranks[ranks.len() - 2]),
        }
    };

    match suits.len() {
        1 => match have_straight {
            true => return Some(StraightFlush(straight_high_card.unwrap())),
            false => return Some(Flush(ranks[4], ranks[3], ranks[2], ranks[1], ranks[0])),
        },
        _ => match have_straight {
            true => return Some(Straight(straight_high_card.unwrap())),
            false => return None,
        },
    }
}

fn best_hand(s: &str) -> Hand {
    let cards = Vec::from_iter(s.split(" ").map(|x| Card::new(x)));
    let maybe_best_suited = suited_and_or_straight_hand(&cards);
    let best_paired = best_paired_hand(&cards);
    println!("{:?}", best_paired);
    println!("{:?}", maybe_best_suited);
    match maybe_best_suited {
        None => best_paired,
        Some(best_suited) =>
            if best_suited > best_paired {
                best_suited
        } else {
            best_paired
        }
    }
}
