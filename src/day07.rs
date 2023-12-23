pub fn run() -> String {
    let mut hands: Vec<Hand> = INPUT.lines().map(|l| l.into()).collect();
    hands.sort_unstable();
    let total = |hands: &Vec<Hand>| hands.iter().enumerate().map(|(n, h)| h.bid * (n + 1)).sum();
    let first: usize = total(&hands);
    for hand in &mut hands {
        hand.joker_count();
    }
    hands.sort_unstable();
    let second = total(&hands);
    format!("{first}\n{second}\n")
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    counts: [u8; 5],
    bid: usize,
}
impl Hand {
    fn joker_count(&mut self) {
        let mut counts = [0; 13];
        for card in &mut self.cards {
            match card.0 {
                9 => card.0 = 0,
                0..=8 => card.0 += 1,
                _ => (),
            }
            counts[card.0 as usize] += 1;
        }
        let jokers = counts[0];
        counts[0] = 0;
        counts.sort_unstable_by_key(|&n| std::cmp::Reverse(n));
        counts[0] += jokers;
        self.counts = counts[0..5].try_into().unwrap();
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.counts.cmp(&other.counts) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.counts == other.counts && self.cards == other.cards
    }
}
impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        use std::array::from_fn;
        let mut counts = [0; 13];
        let mut parts = value.split_whitespace();
        let mut cards = parts.next().unwrap().chars();
        let cards: [Card; 5] = from_fn(|_| cards.next().unwrap().into());
        let bid = parts.next().unwrap().parse().unwrap();

        for card in &cards {
            counts[card.0 as usize] += 1;
        }

        counts.sort_unstable_by_key(|n| std::cmp::Reverse(*n));

        Self {
            cards,
            counts: counts[0..5].try_into().unwrap(),
            bid,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Card(u8);

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            n @ '2'..='9' => Self(n.to_digit(10).unwrap() as u8 - 2),
            'T' => Self(8),
            'J' => Self(9),
            'Q' => Self(10),
            'K' => Self(11),
            'A' => Self(12),
            c => panic!("{c} is not a valid card type"),
        }
    }
}

const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

const INPUT: &str = include_str!("inputs/day07.txt");
