pub fn run() -> String {
    format!("{:?}", parse(TEST))
}

#[derive(Debug)]
struct RaceRecord {
    time: usize,
    distance: usize,
}

fn parse(input: &str) -> Vec<RaceRecord> {
    let mut lines = input.lines();
    let mut races: Vec<_> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| RaceRecord {
            time: n.parse().unwrap(),
            distance: 0,
        })
        .collect();
    races
        .iter_mut()
        .zip(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|n| n.parse().unwrap()),
        )
        .for_each(|(r, n)| r.distance = n);

    races
}
const TEST: &str = "Time:      7  15   30
Distance:  9  40  200";

const INPUT: &str = "Time:        62     73     75     65
Distance:   644   1023   1240   1023";
