pub fn run() -> String {
    let almanac: Almanac = INPUT.into();

    format!(
        "{}\n{}\n",
        almanac.lowest_location(),
        almanac.actual_lowest_location()
    )
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}
impl Almanac {
    fn lowest_location(&self) -> usize {
        self.seeds.iter().map(|s| self.seed_map(*s)).min().unwrap()
    }
    fn seed_map(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |acc, map| {
            match map
                .0
                .iter()
                .find(|r| r.source <= acc && acc < r.source + r.length)
            {
                None => acc,
                Some(range) => range.destination + (acc - range.source),
            }
        })
    }
    fn actual_lowest_location(&self) -> usize {
        use rayon::prelude::*;
        self.seeds
            .par_chunks(2)
            .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .map(|seed| self.seed_map(seed))
            .min()
            .unwrap()
    }
}
impl From<&str> for Almanac {
    fn from(value: &str) -> Self {
        let mut parts = value.split("\n\n");
        let seeds = parts
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();
        Self {
            seeds,
            maps: parts.map(|p| p.into()).collect(),
        }
    }
}
#[derive(Debug)]
struct Map(Vec<Range>);
impl From<&str> for Map {
    fn from(val: &str) -> Self {
        Self(val.lines().skip(1).map(|l| l.into()).collect())
    }
}
#[derive(Debug)]
struct Range {
    destination: usize,
    source: usize,
    length: usize,
}
impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut nums = value.split_whitespace().map(|n| n.parse().unwrap());
        Self {
            destination: nums.next().unwrap(),
            source: nums.next().unwrap(),
            length: nums.next().unwrap(),
        }
    }
}

const INPUT: &str = include_str!("inputs/day05.txt");
const TEST: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
