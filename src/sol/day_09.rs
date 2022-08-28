/*
  --- Day 9: All in a Single Night ---

  Every year, Santa manages to deliver all of his presents in a single night.

  This year, however, he has some new locations to visit; his elves have
  provided him the distances between every pair of locations. He can start and
  end at any two (different) locations he wants, but he must visit each location
  exactly once. What is the shortest distance he can travel to achieve this?

  For example, given the following distances:

  London to Dublin = 464
  London to Belfast = 518
  Dublin to Belfast = 141

  The possible routes are therefore:

  Dublin -> London -> Belfast = 982
  London -> Dublin -> Belfast = 605
  London -> Belfast -> Dublin = 659
  Dublin -> Belfast -> London = 659
  Belfast -> Dublin -> London = 605
  Belfast -> London -> Dublin = 982

  The shortest of these is London -> Dublin -> Belfast = 605, and so the answer
  is 605 in this example.

  What is the distance of the shortest route?

  --- Part Two ---

  The next year, just to show off, Santa decides to take the route with the
  longest distance instead.

  He can still start and end at any two (different) locations he wants, and he
  still must visit each location exactly once.

  For example, given the distances above, the longest route would be 982 via
  (for example) Dublin -> London -> Belfast.

  What is the distance of the longest route?
*/

use aoc2015::lines_from_file;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_09");
    let mut cities: HashMap<String, usize> = HashMap::new();
    let mut graph: Vec<Vec<usize>> = vec![vec![0; 8]; 8];
    for inp in input_lines {
        let c1: Vec<&str> = inp.split(" to ").collect();
        if !cities.contains_key(&c1[0].to_string()) {
            cities.insert(c1[0].to_string(), cities.len());
        }

        let c2: Vec<&str> = c1[1].split(" = ").collect();

        if !cities.contains_key(&c2[0].to_string()) {
            cities.insert(c2[0].to_string(), cities.len());
        }

        graph[cities[&c1[0].to_string()]][cities[&c2[0].to_string()]] =
            c2[1].parse::<usize>().unwrap();
        graph[cities[&c2[0].to_string()]][cities[&c1[0].to_string()]] =
            c2[1].parse::<usize>().unwrap();
    }

    // Print all Cities and Adjacency graph
    // for c in &cities {
    //     println!("{c:?}");
    // }
    // for r in &graph {
    //     for c in r {
    //         print!("{c} ");
    //     }
    //     println!("");
    // }

    part_1(&graph, &cities);
    part_2(&graph, &cities);
}

fn part_1(graph: &Vec<Vec<usize>>, cities: &HashMap<String, usize>) {
    let mut visited: HashSet<usize> = HashSet::<usize>::new();
    let mut res = usize::MAX;
    for i in 0..cities.len() {
        visited.insert(i);
        res = std::cmp::min(res, helper_1(&graph, &mut visited, i, cities.len()));
        visited.clear();
    }

    println!("D09P1: {res}");
}

fn part_2(graph: &Vec<Vec<usize>>, cities: &HashMap<String, usize>) {
    let mut visited: HashSet<usize> = HashSet::<usize>::new();
    let mut res = usize::MIN;
    for i in 0..cities.len() {
        visited.insert(i);
        res = std::cmp::max(res, helper_2(&graph, &mut visited, i, cities.len()));
        visited.clear();
    }

    println!("D09P2: {res}");
}

fn helper_1(
    graph: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    curr: usize,
    total_cities: usize,
) -> usize {
    if visited.len() == total_cities {
        return 0;
    }
    let mut res = usize::MAX;

    for i in 0..graph[curr].len() {
        if graph[curr][i] > 0 && !visited.contains(&i) {
            visited.insert(i);
            res = std::cmp::min(
                res,
                graph[curr][i] + helper_1(&graph, visited, i, total_cities),
            );
            visited.remove(&i);
        }
    }

    return res;
}

fn helper_2(
    graph: &Vec<Vec<usize>>,
    visited: &mut HashSet<usize>,
    curr: usize,
    total_cities: usize,
) -> usize {
    if visited.len() == total_cities {
        return 0;
    }
    let mut res = usize::MIN;

    for i in 0..graph[curr].len() {
        if graph[curr][i] > 0 && !visited.contains(&i) {
            visited.insert(i);
            res = std::cmp::max(
                res,
                graph[curr][i] + helper_2(&graph, visited, i, total_cities),
            );
            visited.remove(&i);
        }
    }

    return res;
}
