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
