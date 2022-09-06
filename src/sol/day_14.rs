/*
  --- Day 14: Reindeer Olympics ---

  This year is the Reindeer Olympics! Reindeer can fly at high speeds, but must
  rest occasionally to recover their energy. Santa would like to know which of
  his reindeer is fastest, and so he has them race.

  Reindeer can only either be flying (always at their top speed) or resting
  (not moving at all), and always spend whole seconds in either state.

  For example, suppose you have the following Reindeer:

      Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
      Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.

  After one second, Comet has gone 14 km, while Dancer has gone 16 km. After
  ten seconds, Comet has gone 140 km, while Dancer has gone 160 km. On the
  eleventh second, Comet begins resting (staying at 140 km), and Dancer continues
  on for a total distance of 176 km. On the 12th second, both reindeer are resting.
  They continue to rest until the 138th second, when Comet flies for another
  ten seconds. On the 174th second, Dancer flies for another 11 seconds.

  In this example, after the 1000th second, both reindeer are resting, and
  Comet is in the lead at 1120 km (poor Dancer has only gotten 1056 km by that point).
  So, in this situation, Comet would win (if the race ended at 1000 seconds).

  Given the descriptions of each reindeer (in your puzzle input), after exactly
  2503 seconds, what distance has the winning reindeer traveled?

  --- Part Two ---

  Seeing how reindeer move in bursts, Santa decides he's not pleased with the
  old scoring system.

  Instead, at the end of each second, he awards one point to the reindeer
  currently in the lead. (If there are multiple reindeer tied for the lead,
  they each get one point.) He keeps the traditional 2503 second time limit,
  of course, as doing otherwise would be entirely ridiculous.

  Given the example reindeer from above, after the first second, Dancer is in
  the lead and gets one point. He stays in the lead until several seconds into
  Comet's second burst: after the 140th second, Comet pulls into the lead and
  gets his first point. Of course, since Dancer had been in the lead for the 139
  seconds before that, he has accumulated 139 points by the 140th second.

  After the 1000th second, Dancer has accumulated 689 points, while poor Comet,
  our old champion, only has 312. So, with the new scoring system, Dancer would
  win (if the race ended at 1000 seconds).

  Again given the descriptions of each reindeer (in your puzzle input), after
  exactly 2503 seconds, how many points does the winning reindeer have?
*/

use aoc2015::lines_from_file;

pub fn run() {
    let input_lines = lines_from_file("./src/input/day_14");
    let mut input: Vec<Vec<i32>> = Vec::new();
    for inp in input_lines {
        let split: Vec<&str> = inp.split_whitespace().collect();
        let speed = split[3].parse::<i32>().unwrap();
        let stamina = split[6].parse::<i32>().unwrap();
        let rest = split[13].parse::<i32>().unwrap();
        input.push(vec![speed, stamina, rest]);
    }

    let total_time = 2503;
    part_1(&input, total_time);
    part_2(&input, total_time);
}

fn part_1(input: &Vec<Vec<i32>>, total_time: i32) {
    let mut res = i32::MIN;
    for inp in input {
        res = std::cmp::max(res, get_distance(inp[0], inp[1], inp[2], total_time));
    }

    println!("D14P1: {res}");
}

fn part_2(input: &Vec<Vec<i32>>, total_time: i32) {
    let mut res = i32::MIN;
    let mut dist_arr: Vec<Vec<i32>> = Vec::new();
    for inp in input {
        dist_arr.push(get_distance_array(inp[0], inp[1], inp[2], total_time));
    }

    let mut points: Vec<i32> = vec![0; dist_arr.len()];

    for i in 0..dist_arr[0].len() {
        let mut max_dist = dist_arr[0][i];
        let mut deer = vec![0];
        // At each second find who is/are at lead and increament point for that deer.
        for d in 1..dist_arr.len() {
            if max_dist < dist_arr[d][i] {
                max_dist = dist_arr[d][i];
                deer.clear();
                deer.push(d);
            } else if max_dist == dist_arr[d][i] {
                deer.push(d);
            }
        }
        for d in deer {
            points[d] += 1;
        }
    }

    for p in points {
        res = std::cmp::max(res, p);
    }

    println!("D14P2: {res}");
}
fn get_distance(speed: i32, stamina: i32, rest: i32, total_time: i32) -> i32 {
    let mut res: i32 = 0;
    let mut time_ = total_time;
    while time_ - stamina >= 0 {
        res += speed * stamina;
        time_ = time_ - rest - stamina;
    }
    if time_ > 0 {
        res += speed * time_;
    }
    return res;
}

fn get_distance_array(speed: i32, stamina: i32, rest: i32, total_time: i32) -> Vec<i32> {
    // Returns array of distance traveled at every second.
    let mut res: Vec<i32> = Vec::new();
    let mut time_ = total_time;
    while time_ - stamina > 0 {
        for _ in 0..stamina {
            if res.len() == 0 {
                res.push(speed);
            } else {
                res.push(res[res.len() - 1] + speed);
            }
        }
        for _ in 0..rest {
            if res.len() >= total_time.try_into().unwrap() {
                break;
            }
            res.push(res[res.len() - 1]);
        }
        time_ = time_ - rest - stamina;
    }

    while time_ > 0 {
        res.push(res[res.len() - 1] + speed);
        time_ -= 1;
    }
    return res;
}
