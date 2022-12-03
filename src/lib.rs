mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

type Problem<'a> = &'a (dyn Fn(&str) -> String + Sync);

static PROBLEMS: [Problem; 50] = [
    &day1::problem1,
    &day1::problem2,
    &day2::problem1,
    &day2::problem2,
    &day3::problem1,
    &day3::problem2,
    &day4::problem1,
    &day4::problem2,
    &day5::problem1,
    &day5::problem2,
    &day6::problem1,
    &day6::problem2,
    &day7::problem1,
    &day7::problem2,
    &day8::problem1,
    &day8::problem2,
    &day9::problem1,
    &day9::problem2,
    &day10::problem1,
    &day10::problem2,
    &day11::problem1,
    &day11::problem2,
    &day12::problem1,
    &day12::problem2,
    &day13::problem1,
    &day13::problem2,
    &day14::problem1,
    &day14::problem2,
    &day15::problem1,
    &day15::problem2,
    &day16::problem1,
    &day16::problem2,
    &day17::problem1,
    &day17::problem2,
    &day18::problem1,
    &day18::problem2,
    &day19::problem1,
    &day19::problem2,
    &day20::problem1,
    &day20::problem2,
    &day21::problem1,
    &day21::problem2,
    &day22::problem1,
    &day22::problem2,
    &day23::problem1,
    &day23::problem2,
    &day24::problem1,
    &day24::problem2,
    &day25::problem1,
    &day25::problem2,
];

pub fn run_problem(day: u8, problem: u8, input: &str) -> String {
    let index = ((day - 1) * 2 + problem - 1) as usize;
    PROBLEMS[index](input)
}
