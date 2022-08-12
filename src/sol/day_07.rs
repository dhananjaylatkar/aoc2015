/*
  --- Day 7: Some Assembly Required ---

  This year, Santa brought little Bobby Tables a set of wires and bitwise
  logic gates! Unfortunately, little Bobby is a little under the recommended
  age range, and he needs help assembling the circuit.

  Each wire has an identifier (some lowercase letters) and can carry a 16-bit
  signal (a number from 0 to 65535). A signal is provided to each wire by a
  gate, another wire, or some specific value. Each wire can only get a signal
  from one source, but can provide its signal to multiple destinations. A gate
  provides no signal until all of its inputs have a signal.

  The included instructions booklet describes how to connect the parts
  together: x AND y -> z means to connect wires x and y to an AND gate,
  and then connect its output to wire z.

  For example:
      123 -> x means that the signal 123 is provided to wire x.
      x AND y -> z means that the bitwise AND of wire x and wire y is provided
      to wire z.
      p LSHIFT 2 -> q means that the value from wire p is left-shifted by 2
      and then provided to wire q.
      NOT e -> f means that the bitwise complement of the value from wire e
      is provided to wire f.

  Other possible gates include OR (bitwise OR) and RSHIFT (right-shift). If,
  for some reason, you'd like to emulate the circuit instead, almost all
  programming languages (for example, C, JavaScript, or Python) provide
  operators for these gates.

  For example, here is a simple circuit:
  123 -> x
  456 -> y
  x AND y -> d
  x OR y -> e
  x LSHIFT 2 -> f
  y RSHIFT 2 -> g
  NOT x -> h
  NOT y -> i

  After it is run, these are the signals on the wires:
  d: 72
  e: 507
  f: 492
  g: 114
  h: 65412
  i: 65079
  x: 123
  y: 456

  In little Bobby's kit's instructions booklet (provided as your puzzle input),
  what signal is ultimately provided to wire a?

  --- Part Two ---

  Now, take the signal you got on wire a, override wire b to that signal, and
  reset the other wires (including wire a). What new signal is ultimately
  provided to wire a?
*/

use aoc2015::lines_from_file;
use std::collections::HashMap;

pub fn run() {
    let input_lines: Vec<String> = lines_from_file("./src/input/day_07");
    let mut graph: HashMap<String, String> = HashMap::new();
    for inp in input_lines {
        let entry: Vec<&str> = inp.split(" -> ").collect();
        graph.insert(entry[1].to_string(), entry[0].to_string());
    }
    let mut memo: HashMap<String, i32> = HashMap::new();
    let p1 = helper(&graph, "a".to_string(), &mut memo);

    *graph.get_mut("b").unwrap() = p1.to_string();
    memo.clear();
    let p2 = helper(&graph, "a".to_string(), &mut memo);

    println!("D07P1: {p1}");
    println!("D07P2: {p2}");
}

fn helper(input: &HashMap<String, String>, curr: String, memo: &mut HashMap<String, i32>) -> i32 {
    match curr.parse::<i32>() {
        Ok(n) => return n,
        _ => {}
    }

    match memo.get(&curr) {
        Some(&n) => return n,
        _ => {}
    }

    let equation: Vec<&str> = input[&curr].split_whitespace().collect();
    let mut res: i32 = 0;
    match equation.len() {
        1 => {
            // Direct signal
            res = helper(input, equation[0].to_string(), memo);
        }
        2 => {
            // NOT gate
            res = !helper(input, equation[1].to_string(), memo);
        }
        3 => {
            // AND, OR, RSHIFT, LSHIFT
            match equation[1] {
                "AND" => {
                    res = helper(input, equation[0].to_string(), memo)
                        & helper(input, equation[2].to_string(), memo);
                }
                "OR" => {
                    res = helper(input, equation[0].to_string(), memo)
                        | helper(input, equation[2].to_string(), memo);
                }
                "RSHIFT" => {
                    res = helper(input, equation[0].to_string(), memo)
                        >> helper(input, equation[2].to_string(), memo);
                }
                "LSHIFT" => {
                    res = helper(input, equation[0].to_string(), memo)
                        << helper(input, equation[2].to_string(), memo);
                }
                _ => {}
            }
        }
        _ => {}
    }
    memo.insert(curr, res);
    return res;
}
