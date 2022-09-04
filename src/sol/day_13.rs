/*
  --- Day 13: Knights of the Dinner Table ---

  In years past, the holiday feast with your family hasn't gone so well. Not
  everyone gets along! This year, you resolve, will be different. You're going
  to find the optimal seating arrangement and avoid all those awkward conversations.

  You start by writing up a list of everyone invited and the amount their
  happiness would increase or decrease if they were to find themselves sitting
  next to each other person. You have a circular table that will be just big
  enough to fit everyone comfortably, and so each person will have exactly two
  neighbors.

  For example, suppose you have only four attendees planned, and you calculate
  their potential happiness as follows:

  Alice would gain 54 happiness units by sitting next to Bob.
  Alice would lose 79 happiness units by sitting next to Carol.
  Alice would lose 2 happiness units by sitting next to David.
  Bob would gain 83 happiness units by sitting next to Alice.
  Bob would lose 7 happiness units by sitting next to Carol.
  Bob would lose 63 happiness units by sitting next to David.
  Carol would lose 62 happiness units by sitting next to Alice.
  Carol would gain 60 happiness units by sitting next to Bob.
  Carol would gain 55 happiness units by sitting next to David.
  David would gain 46 happiness units by sitting next to Alice.
  David would lose 7 happiness units by sitting next to Bob.
  David would gain 41 happiness units by sitting next to Carol.

  Then, if you seat Alice next to David, Alice would lose 2 happiness units
  (because David talks so much), but David would gain 46 happiness units
  (because Alice is such a good listener), for a total change of 44.

  If you continue around the table, you could then seat Bob next to Alice
  (Bob gains 83, Alice gains 54). Finally, seat Carol, who sits next to Bob
  (Carol gains 60, Bob loses 7) and David (Carol gains 55, David gains 41).
  The arrangement looks like this:

       +41 +46
  +55   David    -2
  Carol       Alice
  +60    Bob    +54
       -7  +83

  After trying every other seating arrangement in this hypothetical scenario,
  you find that this one is the most optimal, with a total change in happiness
  of 330.

  What is the total change in happiness for the optimal seating arrangement of
  the actual guest list?

  --- Part Two ---

  In all the commotion, you realize that you forgot to seat yourself.
  At this point, you're pretty apathetic toward the whole thing, and your
  happiness wouldn't really go up or down regardless of who you sit next to.
  You assume everyone else would be just as ambivalent about sitting next to
  you, too.

  So, add yourself to the list, and give all happiness relationships that
  involve you a score of 0.

  What is the total change in happiness for the optimal seating arrangement
  that actually includes yourself?
*/

use aoc2015::lines_from_file;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_13");
    let mut guests: HashMap<String, usize> = HashMap::new();
    let mut graph: Vec<Vec<i64>> = vec![vec![0; 9]; 9];
    for inp in input_lines {
        let g: Vec<&str> = inp.split_whitespace().collect();
        if !guests.contains_key(&g[0].to_string()) {
            guests.insert(g[0].to_string(), guests.len());
        }
        let last = g[g.len() - 1];
        let g2 = &last[0..last.len() - 1];
        if !guests.contains_key(&g2.to_string()) {
            guests.insert(g2.to_string(), guests.len());
        }

        let mut happines = g[3].parse::<i64>().unwrap();
        if g[2] == "lose" {
            happines *= -1;
        }
        graph[guests[&g[0].to_string()]][guests[&g2.to_string()]] = happines;
    }

    // Print all guests and Adjacency graph
    // for c in &guests {
    //     println!("{c:?}");
    // }
    // for r in &graph {
    //     for c in r {
    //         print!("{c} ");
    //     }
    //     println!("");
    // }

    part_1(&graph, &guests);

    // Add one more guest
    guests.insert("me".to_string(), guests.len());
    part_2(&graph, &guests);
}

fn part_1(graph: &Vec<Vec<i64>>, guests: &HashMap<String, usize>) {
    let mut visited: HashSet<usize> = HashSet::<usize>::new();
    let mut res = i64::MIN;
    for i in 0..guests.len() {
        visited.insert(i);
        res = std::cmp::max(res, helper_1(&graph, &mut visited, i, i, guests.len()));
        visited.clear();
    }

    println!("D13P1: {res}");
}
fn part_2(graph: &Vec<Vec<i64>>, guests: &HashMap<String, usize>) {
    let mut visited: HashSet<usize> = HashSet::<usize>::new();
    let mut res = i64::MIN;
    for i in 0..guests.len() {
        visited.insert(i);
        res = std::cmp::max(res, helper_2(&graph, &mut visited, i, i, guests.len()));
        visited.clear();
    }

    println!("D13P2: {res}");
}
fn helper_1(
    graph: &Vec<Vec<i64>>,
    visited: &mut HashSet<usize>,
    initial: usize,
    curr: usize,
    total_guests: usize,
) -> i64 {
    if visited.len() == total_guests {
        return graph[curr][initial] + graph[initial][curr];
    }
    let mut res: i64 = i32::MIN.into();

    for i in 0..graph[curr].len() {
        if graph[curr][i] > 0 && !visited.contains(&i) {
            visited.insert(i);
            res = std::cmp::max(
                res,
                graph[curr][i]
                    + graph[i][curr]
                    + helper_1(&graph, visited, initial, i, total_guests),
            );
            visited.remove(&i);
        }
    }

    return res;
}

fn helper_2(
    graph: &Vec<Vec<i64>>,
    visited: &mut HashSet<usize>,
    initial: usize,
    curr: usize,
    total_guests: usize,
) -> i64 {
    if visited.len() == total_guests {
        return graph[curr][initial] + graph[initial][curr];
    }
    let mut res: i64 = i32::MIN.into();

    for i in 0..graph[curr].len() {
        // Include 0 for last guest but not with themselves
        if (curr == total_guests - 1 || i == total_guests - 1 || graph[curr][i] > 0)
            && !visited.contains(&i)
        {
            visited.insert(i);
            res = std::cmp::max(
                res,
                graph[curr][i]
                    + graph[i][curr]
                    + helper_2(&graph, visited, initial, i, total_guests),
            );
            visited.remove(&i);
        }
    }

    return res;
}
