pub fn run() -> String {
    todo!()
}

struct Hand {
    cards: [Card; 5],
    counts: [u8; 5],
    bid: usize,
}
impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N(u8),
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            n @ '0'..='9' => Self::N(n.to_digit(10).unwrap() as u8),
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            c => panic!("{c} is not a valid card type"),
        }
    }
}

const TEST: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
