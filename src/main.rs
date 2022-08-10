use clap::Parser;
mod sol;

/// Solutions for Advent of Code 2015
/// (https://adventofcode.com/2015)
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   /// AoC Day [0 to 25 inclusive]. 0 for all Days.
   #[clap(short, long, value_parser, default_value_t = 0)]
   day: u8,
}

fn main() {
    let args = Args::parse();

    if args.day > 25 {
        println!("Please provide Day between 0 and 25.");
        return;
    }

    match args.day {
        0 => {
            sol::day_01::run();
        },
        1 => sol::day_01::run(),
        _ => println!("Solution not yet available for {}.", args.day),
    }
}