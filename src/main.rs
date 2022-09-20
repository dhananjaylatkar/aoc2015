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
            sol::day_02::run();
            sol::day_03::run();
            sol::day_04::run();
            sol::day_05::run();
            sol::day_06::run();
            sol::day_07::run();
            sol::day_08::run();
            sol::day_09::run();
            sol::day_10::run();
            sol::day_11::run();
            sol::day_12::run();
            sol::day_13::run();
            sol::day_14::run();
            sol::day_15::run();
            sol::day_16::run();
        }
        1 => sol::day_01::run(),
        2 => sol::day_02::run(),
        3 => sol::day_03::run(),
        4 => sol::day_04::run(),
        5 => sol::day_05::run(),
        6 => sol::day_06::run(),
        7 => sol::day_07::run(),
        8 => sol::day_08::run(),
        9 => sol::day_09::run(),
        10 => sol::day_10::run(),
        11 => sol::day_11::run(),
        12 => sol::day_12::run(),
        13 => sol::day_13::run(),
        14 => sol::day_14::run(),
        15 => sol::day_15::run(),
        16 => sol::day_16::run(),
        _ => println!("Solution not yet available for {}.", args.day),
    }
}
