/*
  --- Day 15: Science for Hungry People ---
  Today, you set out on the task of perfecting your milk-dunking cookie recipe.
  All you have to do is find the right balance of ingredients.

  Your recipe leaves room for exactly 100 teaspoons of ingredients. You make a
  list of the remaining ingredients you could use to finish the recipe (your
      puzzle input) and their properties per teaspoon:

  capacity (how well it helps the cookie absorb milk)
  durability (how well it keeps the cookie intact when full of milk)
  flavor (how tasty it makes the cookie)
  texture (how it improves the feel of the cookie)
  calories (how many calories it adds to the cookie)
  You can only measure ingredients in whole-teaspoon amounts accurately, and
  you have to be accurate so you can reproduce your results in the future.
  The total score of a cookie can be found by adding up each of the properties
  (negative totals become 0) and then multiplying together everything except calories.

  For instance, suppose you have these two ingredients:

  Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
  Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
  Then, choosing to use 44 teaspoons of butterscotch and 56 teaspoons of
  cinnamon (because the amounts of each ingredient must add up to 100) would
  result in a cookie with the following properties:

  A capacity of 44*-1 + 56*2 = 68
  A durability of 44*-2 + 56*3 = 80
  A flavor of 44*6 + 56*-2 = 152
  A texture of 44*3 + 56*-1 = 76
  Multiplying these together (68 * 80 * 152 * 76, ignoring calories for now)
  results in a total score of 62842880, which happens to be the best score
  possible given these ingredients. If any properties had produced a negative
  total, it would have instead become zero, causing the whole score to multiply to zero.

  Given the ingredients in your kitchen and their properties, what is the total
  score of the highest-scoring cookie you can make?

  --- Part Two ---
  Your cookie recipe becomes wildly popular! Someone asks if you can make another
  recipe that has exactly 500 calories per cookie (so they can use it as a meal
  replacement). Keep the rest of your award-winning process the same
  (100 teaspoons, same ingredients, same scoring system).

  For example, given the ingredients above, if you had instead selected 40
  teaspoons of butterscotch and 60 teaspoons of cinnamon (which still adds to
      100), the total calorie count would be 40*8 + 60*3 = 500. The total score
      would go down, though: only 57600000, the best you can do in such trying
      circumstances.

  Given the ingredients in your kitchen and their properties, what is the total
  score of the highest-scoring cookie you can make with a calorie total of 500?
*/

use aoc2015::lines_from_file;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_15");
    let mut input: Vec<Vec<i32>> = Vec::new();

    for inp in input_lines {
        let tmp: Vec<&str> = inp.split_whitespace().collect();
        input.push(vec![
            tmp[2][0..tmp[2].len() - 1].parse::<i32>().unwrap(),
            tmp[4][0..tmp[4].len() - 1].parse::<i32>().unwrap(),
            tmp[6][0..tmp[6].len() - 1].parse::<i32>().unwrap(),
            tmp[8][0..tmp[8].len() - 1].parse::<i32>().unwrap(),
            tmp[10].parse::<i32>().unwrap(),
        ])
    }

    part_1_2(&input);
}

fn part_1_2(input: &Vec<Vec<i32>>) {
    let mut res1 = 0;
    let mut res2 = 0;

    for i in 0..101 {
        for j in 0..101 - i {
            for k in 0..101 - i - j {
                let l = 100 - i - j - k;
                let score_cap =
                    input[0][0] * i + input[1][0] * j + input[2][0] * k + input[3][0] * l;
                let score_dur =
                    input[0][1] * i + input[1][1] * j + input[2][1] * k + input[3][1] * l;
                let score_fla =
                    input[0][2] * i + input[1][2] * j + input[2][2] * k + input[3][2] * l;
                let score_tex =
                    input[0][3] * i + input[1][3] * j + input[2][3] * k + input[3][3] * l;
                let score_cal =
                    input[0][4] * i + input[1][4] * j + input[2][4] * k + input[3][4] * l;

                if score_cap > 0 && score_dur > 0 && score_fla > 0 && score_tex > 0 {
                    let total_score = score_cap * score_dur * score_fla * score_tex;
                    res1 = std::cmp::max(res1, total_score);
                    if score_cal == 500 {
                        res2 = std::cmp::max(res2, total_score);
                    }
                }
            }
        }
    }

    println!("D15P1: {res1}");
    println!("D15P2: {res2}");
}
