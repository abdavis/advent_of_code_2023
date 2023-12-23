pub fn run() -> String {
    let races = parse(INPUT);
    let num: usize = races.iter().map(|r| r.count_winning()).product();
    let races = RaceRecord::fix_kerning(races);
    format!("{}\n{}\n", num, races.count_winning())
}

#[derive(Debug)]
struct RaceRecord {
    time: usize,
    distance: usize,
}

impl RaceRecord {
    fn count_winning(&self) -> usize {
        (0..self.time)
            .skip(1)
            .filter(|n| n * (self.time - n) > self.distance)
            .count()
    }
    fn fix_kerning(input: Vec<Self>) -> Self {
        let mut time = String::new();
        let mut distance = String::new();
        for race in input {
            time += &format!("{}", race.time);
            distance += &format!("{}", race.distance);
        }
        let time = time.parse().unwrap();
        let distance = distance.parse().unwrap();
        Self { time, distance }
    }
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
