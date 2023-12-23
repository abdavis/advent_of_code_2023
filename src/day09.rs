pub fn run() -> String {
    let difs: Vec<Differences> = INPUT.lines().map(|l| l.into()).collect();
    let triangle_diffs: Vec<TriangleDifferences> = difs.into_iter().map(|d| d.into()).collect();
    let next_sum: isize = triangle_diffs.iter().map(|t| t.extrapolate_right()).sum();
    let previous_sum: isize = triangle_diffs.iter().map(|t| t.extrapolate_left()).sum();

    format!("{next_sum}\n{previous_sum}\n")
}

#[derive(Debug)]
struct TriangleDifferences(Vec<Differences>);
impl TriangleDifferences {
    fn extrapolate_right(&self) -> isize {
        let mut current_dif = 0;
        for dif in self.0.iter().rev() {
            current_dif = dif.0.last().unwrap() + current_dif;
        }

        current_dif
    }
    fn extrapolate_left(&self) -> isize {
        let mut current_dif = 0;
        for dif in self.0.iter().rev() {
            current_dif = dif.0.first().unwrap() - current_dif
        }

        current_dif
    }
}
impl From<Differences> for TriangleDifferences {
    fn from(value: Differences) -> Self {
        let mut all_zero = true;
        let difs = value
            .0
            .windows(2)
            .map(|w| {
                let dif = w[1] - w[0];
                if dif != 0 {
                    all_zero = false
                }
                dif
            })
            .collect();
        if all_zero {
            Self(vec![value, Differences(difs)])
        } else {
            let mut vals = Self::from(Differences(difs));
            vals.0.insert(0, value);
            vals
        }
    }
}
#[derive(Debug)]
struct Differences(Vec<isize>);

impl From<&str> for Differences {
    fn from(value: &str) -> Self {
        Self(
            value
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        )
    }
}

const INPUT: &str = include_str!("inputs/day09.txt");
const TEST: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
