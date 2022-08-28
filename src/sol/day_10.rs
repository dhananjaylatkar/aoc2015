/*
  --- Day 10: Elves Look, Elves Say ---

  Today, the Elves are playing a game called look-and-say. They take turns
  making sequences by reading aloud the previous sequence and using that
  reading as the next sequence. For example, 211 is read as "one two, two ones",
  which becomes 1221 (1 2, 2 1s).

  Look-and-say sequences are generated iteratively, using the previous value
  as input for the next step. For each step, take the previous value, and
  replace each run of digits (like 111) with the number of digits (3) followed
  by the digit itself (1).

  For example:

      1 becomes 11 (1 copy of digit 1).
      11 becomes 21 (2 copies of digit 1).
      21 becomes 1211 (one 2 followed by one 1).
      1211 becomes 111221 (one 1, one 2, and two 1s).
      111221 becomes 312211 (three 1s, two 2s, and one 1).

  Starting with the digits in your puzzle input, apply this process 40 times.
  What is the length of the result?

  --- Part Two ---

  Neat, right? You might also enjoy hearing John Conway talking about this
  sequence (that's Conway of Conway's Game of Life fame).

  Now, starting again with the digits in your puzzle input, apply this process
  50 times. What is the length of the new result?
*/

/***************************
 * Very Inefficient Solution, takes ~7mins.
 ***************************/

use aoc2015::lines_from_file;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_10");
    part_1_2(&input_lines[0]);
}

fn part_1_2(input: &String) {
    let mut res: String = input.to_string();

    for _ in 0..40 {
        res = helper(res);
    }
    println!("D10P1: {}", res.len());

    for _ in 0..10 {
        res = helper(res);
    }
    println!("D10P2: {}", res.len());
}

fn helper(input: String) -> String {
    let mut res: String = "".to_string();
    let inp: Vec<char> = input.chars().collect();

    let mut i = 0;

    while i < inp.len() {
        let c = inp[i];
        let mut count = 0;

        while i < inp.len() && inp[i] == c {
            count += 1;
            i += 1;
        }

        res = format!("{res}{count}{c}");
    }
    return res;
}
