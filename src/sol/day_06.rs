/*
  --- Day 6: Probably a Fire Hazard ---
  Because your neighbors keep defeating you in the holiday house decorating
  contest year after year, you've decided to deploy one million lights in a
  1000x1000 grid.

  Furthermore, because you've been especially nice this year, Santa has mailed
  you instructions on how to display the ideal lighting configuration.

  Lights in your grid are numbered from 0 to 999 in each direction; the lights
  at each corner are at 0,0, 0,999, 999,999, and 999,0. The instructions include
  whether to turn on, turn off, or toggle various inclusive ranges given as
  coordinate pairs. Each coordinate pair represents opposite corners of a rectangle,
  inclusive; a coordinate pair like 0,0 through 2,2 therefore refers to 9
  lights in a 3x3 square. The lights all start turned off.

  To defeat your neighbors this year, all you have to do is set up your lights
  by doing the instructions Santa sent you in order.

  For example:
  turn on 0,0 through 999,999 would turn on (or leave on) every light.
  toggle 0,0 through 999,0 would toggle the first line of 1000 lights, turning
  off the ones that were on, and turning on the ones that were off.
  turn off 499,499 through 500,500 would turn off (or leave off) the middle
  four lights.
  After following the instructions, how many lights are lit?

  --- Part Two ---
  You just finish implementing your winning light pattern when you realize you
  mistranslated Santa's message from Ancient Nordic Elvish.

  The light grid you bought actually has individual brightness controls; each
  light can have a brightness of zero or more. The lights all start at zero.

  The phrase turn on actually means that you should increase the brightness of
  those lights by 1.

  The phrase turn off actually means that you should decrease the brightness of
  those lights by 1, to a minimum of zero.

  The phrase toggle actually means that you should increase the brightness of
  those lights by 2.

  What is the total brightness of all lights combined after following Santa's
  instructions?

  For example:
  turn on 0,0 through 0,0 would increase the total brightness by 1.
  toggle 0,0 through 999,999 would increase the total brightness by 2000000.
*/

use aoc2015::lines_from_file;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_06");
    part_1(&input_lines);
    part_2(&input_lines);
}

fn part_1(input: &Vec<String>) {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];
    let mut count: i32 = 0;
    for line in input {
        let sp: Vec<&str> = line.split_whitespace().collect();
        if sp[0] == "toggle" {
            let top: Vec<usize> = sp[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let bottom: Vec<usize> = sp[3]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            count += toggle_1(&mut grid, top[0], top[1], bottom[0], bottom[1]);
        } else {
            let top: Vec<usize> = sp[2]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let bottom: Vec<usize> = sp[4]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if sp[1] == "on" {
                count += turn_on_1(&mut grid, top[0], top[1], bottom[0], bottom[1]);
            } else {
                count += turn_off_1(&mut grid, top[0], top[1], bottom[0], bottom[1]);
            }
        }
    }
    println!("D06P1: {count}");
}

fn part_2(input: &Vec<String>) {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    let mut count: i32 = 0;
    for line in input {
        let sp: Vec<&str> = line.split_whitespace().collect();
        if sp[0] == "toggle" {
            let top: Vec<usize> = sp[1]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let bottom: Vec<usize> = sp[3]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            count += toggle_2(&mut grid, top[0], top[1], bottom[0], bottom[1]);
        } else {
            let top: Vec<usize> = sp[2]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let bottom: Vec<usize> = sp[4]
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if sp[1] == "on" {
                count += turn_on_2(&mut grid, top[0], top[1], bottom[0], bottom[1]);
            } else {
                count += turn_off_2(&mut grid, top[0], top[1], bottom[0], bottom[1]);
            }
        }
    }
    println!("D06P2: {count}");
}

fn toggle_1(grid: &mut Vec<Vec<bool>>, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    let mut count: i32 = 0;
    for i in r1..r2 + 1 {
        for j in c1..c2 + 1 {
            grid[i][j] = !grid[i][j];
            if grid[i][j] {
                count += 1;
            } else {
                count -= 1;
            }
        }
    }
    return count;
}

fn turn_on_1(grid: &mut Vec<Vec<bool>>, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    let mut count: i32 = 0;
    for i in r1..r2 + 1 {
        for j in c1..c2 + 1 {
            if !grid[i][j] {
                count += 1;
                grid[i][j] = true;
            }
        }
    }
    return count;
}

fn turn_off_1(grid: &mut Vec<Vec<bool>>, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    let mut count: i32 = 0;
    for i in r1..r2 + 1 {
        for j in c1..c2 + 1 {
            if grid[i][j] {
                count -= 1;
                grid[i][j] = false;
            }
        }
    }
    return count;
}

fn toggle_2(grid: &mut Vec<Vec<usize>>, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    let mut count: i32 = 0;
    for i in r1..r2 + 1 {
        for j in c1..c2 + 1 {
            grid[i][j] += 2;
            count += 2;
        }
    }
    return count;
}

fn turn_on_2(grid: &mut Vec<Vec<usize>>, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    let mut count: i32 = 0;
    for i in r1..r2 + 1 {
        for j in c1..c2 + 1 {
            grid[i][j] += 1;
            count += 1;
        }
    }
    return count;
}

fn turn_off_2(grid: &mut Vec<Vec<usize>>, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
    let mut count: i32 = 0;
    for i in r1..r2 + 1 {
        for j in c1..c2 + 1 {
            if grid[i][j] > 0 {
                count -= 1;
                grid[i][j] -= 1;
            }
        }
    }
    return count;
}
