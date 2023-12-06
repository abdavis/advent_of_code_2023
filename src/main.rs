mod day01;
mod day02;
mod day03;
mod day04;
fn main() {
    println!("{}", benchmark(day01::run));

    println!("{}", benchmark(day02::run));

    //    println!("{}", benchmark(day03::run));

    println!("{}", benchmark(day04::run));
}
fn benchmark(f: impl Fn() -> String) -> String {
    use std::time::Instant;
    let now = Instant::now();
    let s = f();
    let time = now.elapsed();
    format!("{time:#?}\n{s}")
}
