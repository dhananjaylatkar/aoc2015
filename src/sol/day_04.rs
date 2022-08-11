/*
  --- Day 4: The Ideal Stocking Stuffer ---
  Santa needs help mining some AdventCoins (very similar to bitcoins) to use
  as gifts for all the economically forward-thinking little girls and boys.

  To do this, he needs to find MD5 hashes which, in hexadecimal, start with at
  least five zeroes. The input to the MD5 hash is some secret key
  (your puzzle input, given below) followed by a number in decimal. To mine
  AdventCoins, you must find Santa the lowest positive number (no leading
  zeroes: 1, 2, 3, ...) that produces such a hash.

  For example:
  If your secret key is abcdef, the answer is 609043, because the MD5 hash
  of abcdef609043 starts with five zeroes (000001dbbfa...), and it is the
  lowest such number to do so.
  If your secret key is pqrstuv, the lowest number it combines with to make an
  MD5 hash starting with five zeroes is 1048970; that is, the MD5 hash of
  pqrstuv1048970 looks like 000006136ef....

  --- Part Two ---
  Now find one that starts with six zeroes.
*/

use aoc2015::lines_from_file;
use md5;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_04");
    part_1(&input_lines[0]);
    part_2(&input_lines[0]);
}

fn part_1(input: &String) {
    let mut n: usize = 1;

    loop {
        let curr_secret = format!("{}{}", input, n);
        let hash: String = format!("{:X}", md5::compute(&curr_secret));
        let suffix = &hash[..5];
        if suffix == "00000" {
            println!("D04P1: {n}");
            break;
        }
        n += 1;
    }
}

fn part_2(input: &String) {
    let mut n: usize = 1;

    loop {
        let curr_secret = format!("{}{}", input, n);
        let hash: String = format!("{:X}", md5::compute(&curr_secret));
        let suffix = &hash[..6];
        if suffix == "000000" {
            println!("D04P2: {n}");
            break;
        }
        n += 1;
    }
}
