const INPUT: &str = include_str!("inputs/day02.txt");
pub fn run() -> String {
    let games: Vec<Game> = INPUT.lines().map(|l| l.into()).collect();
    format!(
        "{}\n{}",
        sum_valid(12, 13, 14, &games),
        games.iter().map(|g| g.power()).sum::<usize>()
    )
}

fn sum_valid(r: usize, g: usize, b: usize, games: &Vec<Game>) -> usize {
    games
        .iter()
        .enumerate()
        .filter_map(|(n, game)| {
            if game
                .0
                .iter()
                .all(|d| d.red <= r && d.green <= g && d.blue <= b)
            {
                Some(n + 1)
            } else {
                None
            }
        })
        .sum()
}

struct Game(Vec<Draw>);
impl Game {
    fn power(&self) -> usize {
        let mut max_cubes = Draw::default();
        for draw in &self.0 {
            max_cubes = max_cubes.max(&draw)
        }
        max_cubes.red * max_cubes.green * max_cubes.blue
    }
}
impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let draws = value.split(':').skip(1).next().unwrap();
        Self(draws.split(';').map(|s| s.into()).collect())
    }
}

#[derive(Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}
impl Draw {
    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}
impl From<&str> for Draw {
    fn from(value: &str) -> Self {
        let mut draw = Self::default();
        for val in value.split(',') {
            let val = val.trim();
            let (n, color) = val.split_once(' ').unwrap();
            let n: usize = n.parse().unwrap();
            match color {
                "red" => draw.red = n,
                "green" => draw.green = n,
                "blue" => draw.blue = n,
                s => panic!("{s} isn't a valid color"),
            }
        }

        draw
    }
}
