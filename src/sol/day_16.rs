/*
  --- Day 16: Aunt Sue ---
  Your Aunt Sue has given you a wonderful gift, and you'd like to send her a
  thank you card. However, there's a small problem: she signed it "From, Aunt Sue".

  You have 500 Aunts named "Sue".

  So, to avoid sending the card to the wrong person, you need to figure out
  which Aunt Sue (which you conveniently number 1 to 500, for sanity) gave you
  the gift. You open the present and, as luck would have it, good ol' Aunt Sue
  got you a My First Crime Scene Analysis Machine! Just what you wanted.
  Or needed, as the case may be.

  The My First Crime Scene Analysis Machine (MFCSAM for short) can detect a few
  specific compounds in a given sample, as well as how many distinct kinds of
  those compounds there are. According to the instructions, these are what the
  MFCSAM can detect:

  children, by human DNA age analysis.
  cats. It doesn't differentiate individual breeds.
  Several seemingly random breeds of dog: samoyeds, pomeranians, akitas, and vizslas.
  goldfish. No other kinds of fish.
  trees, all in one group.
  cars, presumably by exhaust or gasoline or something.
  perfumes, which is handy, since many of your Aunts Sue wear a few kinds.
  In fact, many of your Aunts Sue have many of these. You put the wrapping
  from the gift into the MFCSAM. It beeps inquisitively at you a few times and
  then prints out a message on ticker tape:

  children: 3
  cats: 7
  samoyeds: 2
  pomeranians: 3
  akitas: 0
  vizslas: 0
  goldfish: 5
  trees: 3
  cars: 2
  perfumes: 1
  You make a list of the things you can remember about each Aunt Sue. Things
  missing from your list aren't zero - you simply don't remember the value.

  What is the number of the Sue that got you the gift?

  --- Part Two ---
  As you're about to send the thank you note, something in the MFCSAM's
  instructions catches your eye. Apparently, it has an outdated retroencabulator,
  and so the output from the machine isn't exact values - some of them indicate
  ranges.

  In particular, the cats and trees readings indicates that there are greater
  than that many (due to the unpredictable nuclear decay of cat dander and tree
  pollen), while the pomeranians and goldfish readings indicate that there are
  fewer than that many (due to the modial interaction of magnetoreluctance).

  What is the number of the real Aunt Sue?
*/

use aoc2015::lines_from_file;
use std::collections::HashMap;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_16");
    let mut input: Vec<HashMap<String, usize>> = Vec::new();

    for inp in input_lines {
        let tmp: Vec<&str> = inp.split_whitespace().collect();
        let mut items: HashMap<String, usize> = HashMap::new();
        // Sue 1: cars: 9, akitas: 3, goldfish: 0
        items.insert(
            tmp[2][0..tmp[2].len() - 1].to_string(),
            tmp[3][0..tmp[3].len() - 1].parse::<usize>().unwrap(),
        );
        items.insert(
            tmp[4][0..tmp[4].len() - 1].to_string(),
            tmp[5][0..tmp[5].len() - 1].parse::<usize>().unwrap(),
        );
        items.insert(
            tmp[6][0..tmp[6].len() - 1].to_string(),
            tmp[7].parse::<usize>().unwrap(),
        );
        input.push(items);
    }

    let analysis: HashMap<String, usize> = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);
    part_1(&input, &analysis);
    part_2(&input, &analysis);
}

fn part_1(input: &Vec<HashMap<String, usize>>, analysis: &HashMap<String, usize>) {
    for i in 0..input.len() {
        let mut same = true;

        for (key, val) in input[i].iter() {
            if val != &analysis[key] {
                same = false;
                break;
            }
        }

        if same {
            println!("D16P1: {}", i + 1);
            break;
        }
    }
}

fn part_2(input: &Vec<HashMap<String, usize>>, analysis: &HashMap<String, usize>) {
    for i in 0..input.len() {
        let mut same = true;

        for (key, val) in input[i].iter() {
            if key == "cats" || key == "trees" {
                if val <= &analysis[key] {
                    same = false;
                    break;
                }
            } else if key == "pomeranians" || key == "goldfish" {
                if val >= &analysis[key] {
                    same = false;
                    break;
                }
            } else if val != &analysis[key] {
                same = false;
                break;
            }
        }

        if same {
            println!("D16P2: {}", i + 1);
            break;
        }
    }
}
