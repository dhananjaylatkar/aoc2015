/*
  --- Day 17: No Such Thing as Too Much ---
  The elves bought too much eggnog again - 150 liters this time. To fit it all
  into your refrigerator, you'll need to move it into smaller containers.
  You take an inventory of the capacities of the available containers.

  For example, suppose you have containers of size 20, 15, 10, 5, and 5 liters.
  If you need to store 25 liters, there are four ways to do it:

  15 and 10
  20 and 5 (the first 5)
  20 and 5 (the second 5)
  15, 5, and 5
  Filling all containers entirely, how many different combinations of containers
  can exactly fit all 150 liters of eggnog?

  --- Part Two ---
  While playing with all the containers in the kitchen, another load of eggnog
  arrives! The shipping and receiving department is requesting as many containers
  as you can spare.

  Find the minimum number of containers that can exactly fit all 150 liters of
  eggnog. How many different ways can you fill that number of containers and
  still hold exactly 150 litres?

  In the example above, the minimum number of containers was two. There were
  three ways to use that many containers, and so the answer there would be 3.
*/
use aoc2015::lines_from_file;
use std::collections::HashMap;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_17");
    let input: Vec<i32> = input_lines
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    part_1(&input);
    part_2(&input);
}

fn part_1(input: &Vec<i32>) {
    let res = helper_1(&input, 0, 0, 150);
    println!("D17P1: {res}");
}

fn part_2(input: &Vec<i32>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    helper_2(&input, 0, 0, 150, 0, &mut map);
    let mut con_min = i32::MAX;
    let mut res = 0;

    for (k, v) in map.iter() {
        if *k < con_min {
            con_min = *k;
            res = *v;
        }
    }
    println!("D17P2: {res}");
}

fn helper_1(input: &Vec<i32>, idx: usize, container: i32, total_size: i32) -> i32 {
    if container == total_size {
        return 1;
    }

    if idx == input.len() || container > total_size {
        return 0;
    }

    let p1 = helper_1(&input, idx + 1, container + input[idx], total_size);
    let p2 = helper_1(&input, idx + 1, container, total_size);
    return p1 + p2;
}

fn helper_2(
    input: &Vec<i32>,
    idx: usize,
    container: i32,
    total_size: i32,
    number_container: i32,
    map: &mut HashMap<i32, i32>,
) {
    if container == total_size {
        if map.contains_key(&number_container) {
            map.insert(number_container, 1 + map[&number_container]);
        } else {
            map.insert(number_container, 1);
        }
        return;
    }

    if idx == input.len() || container > total_size {
        return;
    }

    helper_2(
        &input,
        idx + 1,
        container + input[idx],
        total_size,
        number_container + 1,
        map,
    );
    helper_2(
        &input,
        idx + 1,
        container,
        total_size,
        number_container,
        map,
    );
    return;
}
