/*
  --- Day 3: Perfectly Spherical Houses in a Vacuum ---

  Santa is delivering presents to an infinite two-dimensional grid of houses.

  He begins by delivering a present to the house at his starting location,
  and then an elf at the North Pole calls him via radio and tells him where to
  move next. Moves are always exactly one house to the north (^), south (v),
  east (>), or west (<). After each move, he delivers another present to the
  house at his new location.

  However, the elf back at the north pole has had a little too much eggnog,
  and so his directions are a little off, and Santa ends up visiting some
  houses more than once. How many houses receive at least one present?

  For example:
    > delivers presents to 2 houses: one at the starting location, and one
      to the east.
    ^>v< delivers presents to 4 houses in a square, including twice to the
      house at his starting/ending location.
    ^v^v^v^v^v delivers a bunch of presents to some very lucky children at
      only 2 houses.

  --- Part Two ---

  The next year, to speed up the process, Santa creates a robot version of
  himself, Robo-Santa, to deliver presents with him.

  Santa and Robo-Santa start at the same location (delivering two presents to
  the same starting house), then take turns moving based on instructions from
  the elf, who is eggnoggedly reading from the same script as the previous year.

  This year, how many houses receive at least one present?

  For example:
    ^v delivers presents to 3 houses, because Santa goes north, and then
      Robo-Santa goes south.
    ^>v< now delivers presents to 3 houses, and Santa and Robo-Santa end up
      back where they started.
    ^v^v^v^v^v now delivers presents to 11 houses, with Santa going one
      direction and Robo-Santa going the other.
*/

use aoc2015::lines_from_file;
use std::collections::HashSet;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_03");
    part_1(&input_lines[0]);
    part_2(&input_lines[0]);
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct House {
    x: i32,
    y: i32,
}
fn part_1(input: &String) {
    let mut set = HashSet::new();
    set.insert(House {x:0, y:0});

    let mut curr_house = House {x:0, y:0};
    for c in input.chars() {
        match c {
            '>' => curr_house.x += 1,
            '<' => curr_house.x -= 1,
            '^' => curr_house.y += 1,
            'v' => curr_house.y -= 1,
            _   => {},
        }
        set.insert(House {x:curr_house.x, y:curr_house.y});
    }

    let houses: usize = set.len();

    println!("D03P1: {houses}");
}

fn part_2(input: &String) {
    let mut set = HashSet::new();
    set.insert(House {x:0, y:0});

    let mut santa_house = House {x:0, y:0};
    let mut robo_house = House {x:0, y:0};
    for (i, c) in input.chars().enumerate() {
        if i%2 == 0 {
            match c {
                '>' => santa_house.x += 1,
                '<' => santa_house.x -= 1,
                '^' => santa_house.y += 1,
                'v' => santa_house.y -= 1,
                _   => {},
            }
            set.insert(House {x:santa_house.x, y:santa_house.y});
        }
        else {
            match c {
                '>' => robo_house.x += 1,
                '<' => robo_house.x -= 1,
                '^' => robo_house.y += 1,
                'v' => robo_house.y -= 1,
                _   => {},
            }
            set.insert(House {x:robo_house.x, y:robo_house.y});
        }
    }

    let houses: usize = set.len();

    println!("D03P2: {houses}");
}
