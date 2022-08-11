/*
  --- Day 5: Doesn't He Have Intern-Elves For This? ---

  Santa needs help figuring out which strings in his text file are naughty or nice.

  A nice string is one with all of the following properties:
      It contains at least three vowels (aeiou only), like aei, xazegov, or
      aeiouaeiouaeiou.
      It contains at least one letter that appears twice in a row, like xx,
      abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
      It does not contain the strings ab, cd, pq, or xy, even if they are part
      of one of the other requirements.

  For example:
      ugknbfddgicrmopn is nice because it has at least three vowels
      (u...i...o...), a double letter (...dd...), and none of the disallowed
      substrings.
      aaa is nice because it has at least three vowels and a double letter,
      even though the letters used by different rules overlap.
      jchzalrnumimnmhp is naughty because it has no double letter.
      haegwjzuvuyypxyu is naughty because it contains the string xy.
      dvszwmarrgswjxmb is naughty because it contains only one vowel.

  How many strings are nice?

  --- Part Two ---

  Realizing the error of his ways, Santa has switched to a better model of
  determining whether a string is naughty or nice. None of the old rules apply,
  as they are all clearly ridiculous.

  Now, a nice string is one with all of the following properties:
      It contains a pair of any two letters that appears at least twice in the
      string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not
      like aaa (aa, but it overlaps).
      It contains at least one letter which repeats with exactly one letter
      between them, like xyx, abcdefeghi (efe), or even aaa.

  For example:
      qjhvhtzxzqqjkmpb is nice because is has a pair that appears twice (qj)
      and a letter that repeats with exactly one letter between them (zxz).
      xxyxx is nice because it has a pair that appears twice and a letter that
      repeats with one between, even though the letters used by each rule overlap.
      uurcxstgmygtbstg is naughty because it has a pair (tg) but no repeat with
      a single letter between them.
      ieodomkazucvgmuy is naughty because it has a repeating letter with one
      between (odo), but no pair that appears twice.

  How many strings are nice under these new rules?
*/
use aoc2015::lines_from_file;
use std::collections::HashSet;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_05");
    part_1(&input_lines);
    part_2(&input_lines);
}

fn part_1(input: &Vec<String>) {
    let mut nice_strings: usize = 0;
    for inp in input {
        let str: Vec<char> = inp.chars().collect();
        if is_good_string_1(&str) {
            nice_strings += 1;
        }
    }
    println!("D05P1: {nice_strings}");
}

fn part_2(input: &Vec<String>) {
    let mut nice_strings: usize = 0;
    for inp in input {
        let str: Vec<char> = inp.chars().collect();
        if is_good_string_2(&str) {
            nice_strings += 1;
        }
    }
    println!("D05P2: {nice_strings}");
}

fn is_good_string_1(str: &Vec<char>) -> bool {
    let mut vov: usize = 0;
    let mut double: bool = false;
    let mut nonostring: bool = false;
    for i in 0..str.len() {
        match str[i] {
            'a' | 'e' | 'i' | 'o' | 'u' => vov += 1,
            'b' => {
                if i != 0 && str[i - 1] == 'a' {
                    nonostring = true;
                    break;
                }
            }
            'd' => {
                if i != 0 && str[i - 1] == 'c' {
                    nonostring = true;
                    break;
                }
            }
            'q' => {
                if i != 0 && str[i - 1] == 'p' {
                    nonostring = true;
                    break;
                }
            }
            'y' => {
                if i != 0 && str[i - 1] == 'x' {
                    nonostring = true;
                    break;
                }
            }
            _ => {}
        }
        if i != 0 && str[i - 1] == str[i] {
            double = true;
        }
    }
    if vov > 2 && double && !nonostring {
        return true;
    }
    return false;
}

fn is_good_string_2(str: &Vec<char>) -> bool {
    let mut double: bool = false;
    let mut step: bool = false;
    let mut set = HashSet::new();

    for i in 1..str.len() - 1 {
        if str[i - 1] == str[i + 1] {
            step = true;
            break;
        }
    }
    let mut i = 0;
    while i < str.len() - 1 {
        let sub = format!("{}{}", str[i], str[i + 1]);
        if  set.contains(&sub) {
            double = true;
            break;
        } else {
            set.insert(sub);
        }
        if i+2 < str.len() && str[i] == str[i + 1] && str[i] == str[i+2] {
            i += 1;
        }
        i += 1;
    }
    if double && step {
        return true;
    }
    return false;
}
