use aoc2022::run_problem;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Advent of Code 2022")]
#[command(author = "Vivekanandan M. <vivekmuthu1910@gmail.com>")]
#[command(about = "Runs the advent of code for the given day and problem", long_about = None)]
struct Args {
    /// Day of the problem
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..26))]
    day: u8,

    /// Problem #
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..3))]
    problem: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let input_file_name = format!("test_data/Day{}.txt", args.day);
    let input_content = std::fs::read_to_string(input_file_name).unwrap();
    match args.problem {
        Some(1) => println!(
            "Day {} Problem 1: {}",
            args.day,
            run_problem(args.day, 1, &input_content)
        ),
        Some(2) => println!(
            "Day {} Problem 2: {}",
            args.day,
            run_problem(args.day, 2, &input_content)
        ),
        None => {
            println!(
                "Day {} Problem 1: {}",
                args.day,
                run_problem(args.day, 1, &input_content)
            );
            println!(
                "Day {} Problem 2: {}",
                args.day,
                run_problem(args.day, 2, &input_content)
            );
        }
        _ => panic!("Wrong Problem number"),
    }
}
