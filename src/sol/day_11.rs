/*
  --- Day 11: Corporate Policy ---

  Santa's previous password expired, and he needs help choosing a new one.

  To help him remember his new password after the old one expires, Santa has
  devised a method of coming up with a password based on the previous one.
  Corporate policy dictates that passwords must be exactly eight lowercase
  letters (for security reasons), so he finds his new password by incrementing
  his old password string repeatedly until it is valid.

  Incrementing is just like counting with numbers: xx, xy, xz, ya, yb, and so
  on. Increase the rightmost letter one step; if it was z, it wraps around to a,
  and repeat with the next letter to the left until one doesn't wrap around.

  Unfortunately for Santa, a new Security-Elf recently started, and he has
  imposed some additional password requirements:

      Passwords must include one increasing straight of at least three letters,
      like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd
      doesn't count.
      Passwords may not contain the letters i, o, or l, as these letters can be
      mistaken for other characters and are therefore confusing.
      Passwords must contain at least two different, non-overlapping pairs of
      letters, like aa, bb, or zz.

  For example:
      hijklmmn meets the first requirement (because it contains the straight hij)
      but fails the second requirement requirement (because it contains i and l).
      abbceffg meets the third requirement (because it repeats bb and ff) but
      fails the first requirement.
      abbcegjk fails the third requirement, because it only has one double
      letter (bb).
      The next password after abcdefgh is abcdffaa.
      The next password after ghijklmn is ghjaabcc, because you eventually
      skip all the passwords that start with ghi..., since i is not allowed.

  Given Santa's current password (your puzzle input), what should his next
  password be?

  --- Part Two ---

  Santa's password expired again. What's the next one?
*/

use aoc2015::lines_from_file;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_11");
    part_1_2(&input_lines[0]);
}

fn part_1_2(input: &String) {
    let mut inp: Vec<u8> = input.bytes().collect();

    get_next(&mut inp);
    while !is_valid(&inp) {
        get_next(&mut inp);
    }
    let res1 = String::from_utf8(inp).unwrap();
    println!("D11P1: {res1}");

    inp = res1.bytes().collect();
    get_next(&mut inp);
    while !is_valid(&inp) {
        get_next(&mut inp);
    }
    let res2 = String::from_utf8(inp).unwrap();
    println!("D11P2: {res2}");
}

fn get_next(input: &mut Vec<u8>) {
    let mut i = input.len() - 1;

    loop {
        input[i] += 1;
        if input[i] > b'z' {
            input[i] = b'a';
            i -= 1;
        } else {
            break;
        }
        if i == 0 {
            break;
        }
    }
}

fn is_valid(input: &Vec<u8>) -> bool {
    let mut c1 = false;
    let mut c3 = false;

    for i in 0..input.len() - 2 {
        if input[i] + 1 == input[i + 1] && input[i + 1] + 1 == input[i + 2] {
            c1 = true;
            break;
        }
    }

    for c in input {
        match c {
            b'i' | b'o' | b'l' => {
                return false;
            }
            _ => {}
        }
    }

    let mut pairs = 0;
    let mut i = 0;
    while i < input.len() - 1 {
        if input[i] == input[i + 1] {
            pairs += 1;
            i += 2;
        } else {
            i += 1;
        }
        if pairs == 2 {
            c3 = true;
        }
    }

    return c1 && c3;
}
