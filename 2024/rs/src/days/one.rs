use std::collections::HashMap;

use crate::util::data;

fn parse_lines_into_vecs(input: String) -> (Vec<i32>, Vec<i32>) {
    let lines: Vec<&str> = input.lines().collect();

    let mut l_ints: Vec<i32> = Vec::new();
    let mut r_ints: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split("   ").collect();
        if let [left, right] = parts[..] {
            l_ints.push(
                left.parse::<i32>()
                    .expect("Failed to parse int from string"),
            );
            r_ints.push(
                right
                    .parse::<i32>()
                    .expect("Failed to parse int from string"),
            );
        } else {
            eprintln!("skipping malformed line: {}", line);
        }
    }
    (l_ints, r_ints)
}
fn part_one(input: String) -> i32 {
    let (mut l_ints, mut r_ints) = parse_lines_into_vecs(input);
    l_ints.sort();
    r_ints.sort();

    let mut i = 0;
    let mut diff = 0;

    while i < l_ints.len() {
        let _diff = l_ints[i] - r_ints[i];
        diff += _diff.abs();
        i += 1;
    }
    diff
}
fn part_two(input: String) -> i32 {
    let (l_ints, r_ints) = parse_lines_into_vecs(input);

    let mut r_map: HashMap<i32, i32> = HashMap::new();
    for n in r_ints {
        r_map
            .entry(n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let mut diff = 0;
    for n in l_ints {
        let r_val = r_map.get(&n);
        if let Some(r_val) = r_val {
            diff += r_val * n;
        }
    }
    diff
}

pub fn day_one() {
    println!("** DAY ONE **");
    let data = data::get_data_from_file("day_one");

    let p_one = part_one(data.to_owned());
    println!("    Part 1: {}", p_one);
    let p_two = part_two(data.to_owned());
    println!("    Part 2: {}", p_two);
}
