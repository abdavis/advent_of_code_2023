pub fn run() -> String {
    let games: Vec<(usize, usize, Game<10, 25>)> =
        INPUT.lines().map(|l| (1, 0, l.into())).collect();

    let (cards, points) = total_cards(games);

    format!("{points}\n{cards}\n")
}

fn total_cards<const W: usize, const M: usize>(
    mut games: Vec<(usize, usize, Game<W, M>)>,
) -> (usize, usize) {
    for i in 0..games.len() {
        for j in (i + 1)..(i + 1 + games[i].2.wins()) {
            games[j].0 += games[i].0
        }
        games[i].1 = match games[i].2.wins() {
            0 => 0,
            n => 2_usize.pow(n as u32 - 1),
        };
    }

    games
        .iter()
        .map(|(n, w, _)| (*n, *w))
        .reduce(|(n, w), (nn, nw)| (n + nn, w + nw))
        .unwrap()
}

struct Game<const W: usize, const M: usize> {
    winning: [usize; W],
    mine: [usize; M],
}

impl<const W: usize, const M: usize> Game<W, M> {
    fn wins(&self) -> usize {
        self.mine
            .iter()
            .map(|n| if self.winning.contains(n) { 1 } else { 0 })
            .sum()
    }
}

impl<const W: usize, const M: usize> From<&str> for Game<W, M> {
    fn from(value: &str) -> Self {
        use std::array::from_fn;
        let game = value.split_once(':').unwrap().1.trim();
        let (wins, mine) = game.split_once('|').unwrap();
        Self {
            winning: {
                let mut wins = wins.split_whitespace().map(|n| n.parse().unwrap());
                from_fn(|_| wins.next().expect("not enough winning numbers"))
            },
            mine: {
                let mut mine = mine.split_whitespace().map(|n| n.parse().unwrap());
                from_fn(|_| mine.next().expect("not enough picked numbers"))
            },
        }
    }
}

const INPUT: &str = include_str!("inputs/day04.txt");
