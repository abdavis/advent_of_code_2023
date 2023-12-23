mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
fn main() {
    println!("{}", benchmark(day01::run));

    println!("{}", benchmark(day02::run));

    //    println!("{}", benchmark(day03::run));

    println!("{}", benchmark(day04::run));

    //    println!("{}", benchmark(day05::run));

    println!("{}", benchmark(day06::run));

    println!("{}", benchmark(day07::run));

    println!("{}", benchmark(day08::run));

    println!("{}", benchmark(day09::run));
}
fn benchmark(f: impl Fn() -> String) -> String {
    use std::time::Instant;
    let now = Instant::now();
    let s = f();
    let time = now.elapsed();
    format!("{time:#?}\n{s}")
}
