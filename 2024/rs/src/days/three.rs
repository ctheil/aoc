use regex::Regex;
use std::fmt::format;

// fn parse_lines_into_reports(input: String) -> Vec<Vec<i32>> {
//     let lines: Vec<&str> = input.lines().collect();
//
//     let mut data: Vec<Vec<i32>> = Vec::new();
//
//     for line in lines {
//         let parts: Vec<&str> = line.split(" ").collect();
//         let mut report: Vec<i32> = Vec::new();
//         for part in parts {
//             report.push(
//                 part.parse::<i32>()
//                     .expect("Failed to parse int from string"),
//             )
//         }
//         data.push(report)
//     }
//     data
// }
//
// struct Mul<'a> {
//     left: Option<i32>,
//     right: Option<i32>,
//     seq: [&'a str; 8],
//     seq_counter: usize,
// }
//
// impl Mul<'_> {
//     fn new(&mut self) -> &Mul {
//         self.seq = ["m", "u", "l", "(", "num", ",", "num", ")"];
//         self.left = None;
//         self.right = None;
//         self.seq_counter = 0;
//         return self;
//     }
//
//     fn seq_ref(&self) -> &str {
//         assert!(self.seq_counter <= self.seq.len(), "index out of bounds.");
//         self.seq[self.seq_counter]
//     }
//     fn check(&mut self, char: &str) {
//         let mul_type = Mul::determine_mul_type(self.seq_ref(), char);
//
//         match mul_type {
//             Char => self.seq_counter += 1,
//             Num => curr_num = format!("{curr_num}{char}"),
//             NextNum => {
//                 let num: i32 = curr_num.parse().expect("Failed to parse num from string");
//                 valid_muls.push()
//             }
//         }
//     }
//
//     fn update_curr_num
//     fn determine_mul_type(seq_ref: &str, char: &str) -> MulType {
//         match seq_ref {
//             "," => MulType::NextNum,
//             "num" => MulType::Num,
//             _ if seq_ref == char => MulType::Char,
//             _ => MulType::Invalid,
//         }
//     }
// }
#[derive(Debug)]
enum MulType {
    Invalid,
    Char,
    Num,
    Terminus,
}
fn determine_mul_type(seq_ref: &str, char: &str) -> MulType {
    // dbg!("Setting mulype for char", char);
    match seq_ref {
        "num" => MulType::Num,
        "," => MulType::Num,
        ")" => MulType::Terminus,
        _ if seq_ref == char => MulType::Char,
        _ => MulType::Invalid,
    }
}

#[derive(Debug)]
enum CurrentNum {
    First,
    Second,
    Finished,
}
#[derive(Debug)]
struct MulNums {
    first: String,
    second: String,
    curr: CurrentNum, // nums: (Option<&str>, Option<&str>),
    nums: Option<(i32, i32)>, // curr: CurrentNum,
                      // finished: bool
}

impl MulNums {
    fn new() -> Self {
        MulNums {
            curr: CurrentNum::First,
            first: "".to_string(),
            nums: None,
            second: "".to_string(),
        }
    }

    fn insert(&mut self, char: &str) -> bool {
        if char == "," {
            self.curr = CurrentNum::Second;
            return false;
        } else if char == ")" {
            self.curr = CurrentNum::Finished
        }
        match &self.curr {
            CurrentNum::First => self.first.push_str(char),
            CurrentNum::Second => self.second.push_str(char),
            CurrentNum::Finished => return true,
        }
        // dbg!(self);

        false
    }

    fn get_nums(&self) -> Result<(i32, i32), &str> {
        if let (Ok(first_int), Ok(second_int)) =
            (self.first.parse::<i32>(), self.second.parse::<i32>())
        {
            Ok((first_int, second_int))
        } else {
            Err("Failed to convert to int touple.")
        }
    }
}

fn derive_valid_muls(input: &str) {
    let seq = ["m", "u", "l", "(", "num", ",", "num", ")"];
    let chars: Vec<&str> = input.split("").collect();
    let mut seq_counter = 0;
    let mut curr = MulNums::new();

    let mut muls: Vec<(i32, i32)> = Vec::new();
    for char in chars {
        let seq_ref = seq[seq_counter];
        let mul_type = determine_mul_type(seq_ref, char);
        // dbg!(&mul_type, char);
        match mul_type {
            MulType::Char => seq_counter += 1,
            MulType::Invalid => {
                seq_counter = 0;
                curr = MulNums::new();
            }
            MulType::Num => {
                curr.insert(char);
                seq_counter += 1;
            }
            MulType::Terminus => {
                // dbg!(&curr);
                if let Ok(tuple) = curr.get_nums() {
                    // dbg!(tuple, &muls);
                    muls.push(tuple);
                    curr = MulNums::new();
                    seq_counter = 0;
                } else {
                    eprintln!("\nError converting strs to ints...\n");
                    curr = MulNums::new();
                    seq_counter = 0;
                }
            }
        }
    }

    // dbg!(muls);
}
fn part_one(data: &str) -> String {
    // todo!();
    derive_valid_muls(data);
    "Part One".to_string()
}
// fn part_two(data: String) -> String {
//     todo!();
//     "Part Two".to_string()
// }
pub fn day_three() {
    // todo!()
    let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    println!("\n** DAY THREE **");
    // let data = data::get_data_from_file("day_two");

    let p_one = part_one(data);
    println!("    Part 1: {}", p_one);
    // let p_two = part_two(data.to_owned());
    // println!("    Part 2: {}", p_two);
}
