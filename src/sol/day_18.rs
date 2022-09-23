/*
--- Day 18: Like a GIF For Your Yard ---

After the million lights incident, the fire code has gotten stricter: now, at
most ten thousand lights are allowed. You arrange them in a 100x100 grid.

Never one to let you down, Santa again mails you instructions on the ideal
lighting configuration. With so few lights, he says, you'll have to resort to animation.

Start by setting your lights to the included initial configuration (your puzzle input).
A # means "on", and a . means "off".

Then, animate your grid in steps, where each step decides the next configuration
based on the current one. Each light's next state (either on or off) depends on
its current state and the current states of the eight lights adjacent to it (including diagonals).
Lights on the edge of the grid might have fewer than eight neighbors; the
missing ones always count as "off".

For example, in a simplified 6x6 grid, the light marked A has the neighbors
numbered 1 through 8, and the light marked B, which is on an edge, only has
the neighbors marked 1 through 5:

1B5...
234...
......
..123.
..8A4.
..765.

The state a light should have next is based on its current state (on or off)
plus the number of neighbors that are on:

    A light which is on stays on when 2 or 3 neighbors are on, and turns off
    otherwise.
    A light which is off turns on if exactly 3 neighbors are on, and stays off
    otherwise.

All of the lights update simultaneously; they all consider the same current
state before moving to the next.

Here's a few steps from an example configuration of another 6x6 grid:

Initial state:
.#.#.#
...##.
#....#
..#...
#.#..#
####..

After 1 step:
..##..
..##.#
...##.
......
#.....
#.##..

After 2 steps:
..###.
......
..###.
......
.#....
.#....

After 3 steps:
...#..
......
...#..
..##..
......
......

After 4 steps:
......
......
..##..
..##..
......
......

After 4 steps, this example has four lights on.

In your grid of 100x100 lights, given your initial configuration, how many
lights are on after 100 steps?

--- Part Two ---

You flip the instructions over; Santa goes on to point out that this is all
just an implementation of Conway's Game of Life. At least, it was, until you
notice that something's wrong with the grid of lights you bought: four lights,
       one in each corner, are stuck on and can't be turned off. The example
       above will actually run like this:

Initial state:
##.#.#
...##.
#....#
..#...
#.#..#
####.#

After 1 step:
#.##.#
####.#
...##.
......
#...#.
#.####

After 2 steps:
#..#.#
#....#
.#.##.
...##.
.#..##
##.###

After 3 steps:
#...##
####.#
..##.#
......
##....
####.#

After 4 steps:
#.####
#....#
...#..
.##...
#.....
#.#..#

After 5 steps:
##.###
.##..#
.##...
.##...
#.#...
##...#

After 5 steps, this example now has 17 lights on.

In your grid of 100x100 lights, given your initial configuration, but with the
four corners always in the on state, how many lights are on after 100 steps?
*/

use aoc2015::lines_from_file;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_18");
    let mut grid_1: Vec<Vec<char>> = Vec::new();
    let mut grid_2: Vec<Vec<char>> = Vec::new();

    for inp in input_lines {
        grid_1.push(inp.chars().collect());
        grid_2.push(inp.chars().collect());
    }

    part_1(&mut grid_1);

    let n = grid_2.len();
    let m = grid_2[0].len();
    grid_2[0][0] = '#';
    grid_2[0][m - 1] = '#';
    grid_2[n - 1][0] = '#';
    grid_2[n - 1][m - 1] = '#';
    part_2(&mut grid_2);
}

fn part_1(grid: &mut Vec<Vec<char>>) {
    for _ in 0..100 {
        *grid = helper_1(&grid);
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut res = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '#' {
                res += 1;
            }
        }
    }

    println!("D18P1: {res}");
}

fn part_2(grid: &mut Vec<Vec<char>>) {
    for _ in 0..100 {
        *grid = helper_2(&grid);
    }

    let n = grid.len();
    let m = grid[0].len();
    let mut res = 0;

    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '#' {
                res += 1;
            }
        }
    }

    println!("D18P2: {res}");
}

fn helper_1(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut res: Vec<Vec<char>> = vec![vec!['.'; m]; n];
    for i in 0..n {
        for j in 0..m {
            let lights = get_lights(&grid, i, j);
            match grid[i][j] {
                '#' => {
                    if lights == 2 || lights == 3 {
                        res[i][j] = '#';
                    }
                }
                _ => {
                    if lights == 3 {
                        res[i][j] = '#';
                    }
                }
            }
        }
    }
    res
}

fn helper_2(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut res: Vec<Vec<char>> = vec![vec!['.'; m]; n];
    for i in 0..n {
        for j in 0..m {
            if (i == 0 && j == 0)
                || (i == 0 && j == m - 1)
                || (i == n - 1 && j == 0)
                || (i == n - 1 && j == m - 1)
            {
                res[i][j] = '#';
                continue;
            }
            let lights = get_lights(&grid, i, j);
            match grid[i][j] {
                '#' => {
                    if lights == 2 || lights == 3 {
                        res[i][j] = '#';
                    }
                }
                _ => {
                    if lights == 3 {
                        res[i][j] = '#';
                    }
                }
            }
        }
    }
    res
}

fn get_lights(grid: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
    let mut res = 0;
    let n = grid.len() as i32;
    let m = grid[0].len() as i32;
    let dirs: Vec<Vec<i32>> = vec![
        vec![0, 1],
        vec![1, 0],
        vec![-1, 0],
        vec![0, -1],
        vec![1, 1],
        vec![-1, -1],
        vec![-1, 1],
        vec![1, -1],
    ];

    for dir in dirs {
        let x_: i32 = x as i32 + dir[0];
        let y_: i32 = y as i32 + dir[1];
        if x_ >= 0 && x_ < n && y_ >= 0 && y_ < m && grid[x_ as usize][y_ as usize] == '#' {
            res += 1;
        }
    }
    return res;
}
