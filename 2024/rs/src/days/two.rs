use crate::util::data;

fn parse_lines_into_reports(input: String) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = input.lines().collect();

    let mut data: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let mut report: Vec<i32> = Vec::new();
        for part in parts {
            report.push(
                part.parse::<i32>()
                    .expect("Failed to parse int from string"),
            )
        }
        data.push(report)
    }
    data
}

#[derive(Debug)]
enum ReportDirection {
    Unknown,
    Asc,
    Desc,
}
fn is_report_direction_constant(report: &Vec<i32>, tolerance: i32) -> bool {
    // dbg!(report, tolerance);
    // println!("Checking direction report: {:?}, {}", report, tolerance);
    let mut direction = ReportDirection::Unknown;
    let mut result = true;

    let mut last: Option<i32> = None;
    let mut i = 0;
    while i < report.len() {
        let x = report[i];

        // for x in report {
        if let Some(last) = last {
            if let ReportDirection::Unknown = direction {
                if last < x {
                    direction = ReportDirection::Asc
                } else {
                    direction = ReportDirection::Desc
                }
                // dbg!("set report direction, {}", &direction);
            }
            result = match direction {
                ReportDirection::Asc => last < x,
                ReportDirection::Desc => last > x,
                _ => unreachable!(),
            };
        }
        if !result {
            if tolerance <= 0 {
                result = false;
                break;
            }
            return is_report_direction_constant(&exclude_index(&report, i), tolerance - 1);
        }
        last = Some(x);
        i += 1;
    }

    result
}

fn is_diff_acceptible(report: &Vec<i32>, tolerance: i32) -> bool {
    // dbg!(report, tolerance);
    let mut result: bool = true;

    let mut last: Option<i32> = None;
    let mut i = 0;
    while i < report.len() {
        let x = report[i];

        if let Some(last) = last {
            let diff = (last - x).abs();
            result = (1..=3).contains(&diff);
            if diff < 1 || diff > 3 {
                println!(
                    "\n\nDIFF FAILED (Attempt {}): {}, {:?}\n\n",
                    tolerance, diff, report
                )
            }
        }
        if !result {
            if tolerance <= 0 {
                result = false;
                break;
            }
            return is_diff_acceptible(&exclude_index(&report, i), tolerance - 1);
        }
        last = Some(x);
        i += 1;
    }

    result
}

fn exclude_index(arr: &[i32], index: usize) -> Vec<i32> {
    assert!(index < arr.len(), "index out of bounds.");
    let mut result = Vec::new();
    let mut i = 0;
    while i < arr.len() {
        if i == index {
            i += 1;
            continue;
        }
        result.push(arr[i]);
        i += 1;
    }
    result
}

/*
The levels are either all increasing or all decreasing.
Any two adjacent levels differ by at least one and at most three.
In the example above, the reports can be found safe or unsafe by checking those rules:

7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
So, in this example, 2 reports are safe.

Analyze the unusual data from the engineers. How many reports are safe?
*/
fn part_one(data: String) -> i32 {
    let tolerance = 0;
    let reports = parse_lines_into_reports(data);

    let mut safe_reports = 0;
    for report in reports {
        let is_report_safe = is_report_direction_constant(&report, tolerance)
            && is_diff_acceptible(&report, tolerance);
        // let is_report_safe = is_report_direction_constant(&report);
        if !is_report_safe {
            dbg!("FAILED REPORT: ", &report);
        }
        if is_report_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

/*
*Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

More of the above example's reports are now safe:

7 6 4 2 1: Safe without removing any level.
1 2 7 8 9: Unsafe regardless of which level is removed.
9 7 6 2 1: Unsafe regardless of which level is removed.
1 3 2 4 5: Safe by removing the second level, 3.
8 6 4 4 1: Safe by removing the third level, 4.
1 3 6 7 9: Safe without removing any level.
Thanks to the Problem Dampener, 4 reports are actually safe!
*/
fn part_two(data: String) -> i32 {
    let tolerance = 1;
    let reports = parse_lines_into_reports(data);

    let mut safe_reports = 0;
    for report in reports {
        // let mut is_report_safe = true;
        let is_dir_const = is_report_direction_constant(&report, tolerance);
        let is_diff_good = is_diff_acceptible(&report, tolerance);

        let is_report_safe = is_dir_const && is_diff_good;

        if is_report_safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

/* GPT IMPL
* */

// fn is_safe_report(report: &String) -> bool {
//     let levels: Vec<i32> = report
//         .split_whitespace()
//         .filter_map(|num| num.parse::<i32>().ok())
//         .collect();
//
//     if levels.len() < 2 {
//         return false;
//     }
//
//     let mut is_inc = true;
//     let mut is_dec = true;
//
//     for i in 0..levels.len() - 1 {
//         let diff = levels[i + 1] - levels[i];
//         if diff < 1 || diff > 3 {
//             return false;
//         }
//
//         if diff > 0 {
//             is_dec = false;
//         }
//         if diff < 0 {
//             is_inc = false
//         }
//     }
//     is_inc || is_dec
// }
// fn gpt_part_one(input: String) -> usize {
//     let safe_reports = input
//         .lines()
//         .into_iter()
//         .filter(|line| is_safe_report(&line.to_string()))
//         .count();
//     println!("Number of safe reports: {}", safe_reports);
//     safe_reports
// }
pub fn day_two() {
    // let data = "8 10 10 11 13".to_string();
    //     let data = "7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9"
    //         .to_string();
    /*
     * In the above example, lines 0, 3, 4, 5 are safe_reports
     * whereas lines 1 and 2 are not safe with a tolerance of 1
     * */

    println!("\n** DAY TWO **");
    let data = data::get_data_from_file("day_two");

    // let p_one = part_one(data.to_owned());
    let p_one = part_one(data.to_owned());
    println!("    Part 1: {}", p_one);
    let p_two = part_two(data.to_owned());
    println!("    Part 2: {}", p_two);
}
