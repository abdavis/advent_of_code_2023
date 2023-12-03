pub fn run() -> String {
    let schematic: Schematic<100> = INPUT.into();
    todo!()
}

struct Part {
    name: Value,
    number: usize,
}

struct Schematic<const N: usize> {
    grid: [[Value; N]; N],
}

impl<const N: usize> Schematic<N> {
    fn parts(&self) -> Vec<Part> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().filter_map(|(c, value)| match value {
                    Value::Empty | Value::Num(_) => None,

                    Value::Symbol(s) => if r > 0 {},
                })
            })
            .collect()
    }
    fn extract_number(&self, row: usize, col: usize) -> usize {}
    fn extract_left(&self, row: usize, col: usize) -> usize {}
    fn extract_right(&self, row: usize, col: usize) -> usize {}
}

impl<const N: usize> From<&str> for Schematic<N> {
    fn from(value: &str) -> Self {
        use std::array::from_fn;
        let mut lines = value.lines();
        Self {
            grid: from_fn(|_| {
                let mut chars = lines
                    .next()
                    .expect("input has too few lines for schematic of size {N}")
                    .chars();
                from_fn(|_| {
                    chars
                        .next()
                        .expect("row had too few characters for schematic of size {N}}")
                        .into()
                })
            }),
        }
    }
}

enum Value {
    Empty,
    Num(u8),
    Symbol(Symbol),
}
enum Symbol {
    Percent,
    Plus,
    Amp,
    At,
    Asterisk,
    Div,
    Pound,
    Equal,
    Dollar,
    Minus,
}
impl From<char> for Value {
    fn from(value: char) -> Self {
        match value {
            c @ '0'..='9' => Self::Num(c.to_digit(10).unwrap() as u8),
            '.' => Self::Empty,
            '%' => Self::Symbol(Symbol::Percent),
            '+' => Self::Symbol(Symbol::Plus),
            '&' => Self::Symbol(Symbol::Amp),
            '@' => Self::Symbol(Symbol::At),
            '*' => Self::Symbol(Symbol::Asterisk),
            '/' => Self::Symbol(Symbol::Div),
            '#' => Self::Symbol(Symbol::Pound),
            '=' => Self::Symbol(Symbol::Equal),
            '$' => Self::Symbol(Symbol::Dollar),
            '-' => Self::Symbol(Symbol::Minus),
            c => panic!("{c} isn't a valid symbol"),
        }
    }
}
const INPUT: &str = include_str!("inputs/day03.txt");
