use core::panic;
use std::collections::HashMap;
pub fn run() -> String {
    let mut parts = INPUT.split("\n\n");
    let instructions: Vec<Instruction> = parts.next().unwrap().chars().map(|c| c.into()).collect();
    let graph: Graph = parts.next().unwrap().into();
    let steps = traverse(&instructions, &graph);
    format!("{steps}\n\n")
}
struct CycleInfo {
    length: usize,
    end_points: Vec<usize>,
}

fn traverse(instructions: &Vec<Instruction>, graph: &Graph) -> usize {
    let mut instructions = instructions.iter().cycle();

    let mut node = "AAA";
    let mut count = 0;
    while node != "ZZZ" {
        match instructions.next().unwrap() {
            Instruction::L => node = graph.0[node].0,
            Instruction::R => node = graph.0[node].1,
        }
        count += 1;
    }

    count
}

#[derive(Debug)]
enum Instruction {
    L,
    R,
}
impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::L,
            'R' => Self::R,
            c => panic!("{c} is not a valid direction"),
        }
    }
}

#[derive(Debug)]
struct Graph<'a>(HashMap<&'a str, (&'a str, &'a str)>);
impl<'a> From<&'a str> for Graph<'a> {
    fn from(value: &'a str) -> Self {
        Self(
            value
                .lines()
                .map(|l| {
                    let mut parts = l.split([' ', '(', ')', ',', '=']).filter(|s| s.len() > 0);
                    (
                        parts.next().unwrap(),
                        (parts.next().unwrap(), parts.next().unwrap()),
                    )
                })
                .collect(),
        )
    }
}

const TEST: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

const INPUT: &str = include_str!("inputs/day08.txt");
