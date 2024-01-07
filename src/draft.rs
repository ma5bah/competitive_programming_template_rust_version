#![allow(unused)]

use std::cmp::max;
use std::collections::HashSet;

use cp::*;
use std::iter::Map;
use std::os::raw::c_double;

mod cp {
    use std::io::Bytes;

    #[derive(Default, Debug)]
    pub struct Scanner {
        buffer: Vec<String>,
    }

    impl Scanner {
        pub fn new() -> Self {
            let mut stream = Self { buffer: vec![] };
            stream.parse();
            stream
        }
        pub fn next<T: std::str::FromStr>(&mut self) -> T {
            match self.buffer.pop() {
                Some(token) => match token.parse::<T>() {
                    Ok(_token) => _token,
                    Err(_) => {
                        panic!("Parsing error.")
                    }
                },
                _ => {
                    panic!("May be buffer is empty.")
                }
            }
        }
        fn parse(&mut self) -> () {
            loop {
                match self.read_lines() {
                    Ok(line) => {
                        let mut str_vec = line
                            .split_whitespace()
                            .map(String::from)
                            .collect::<Vec<String>>();
                        if str_vec.is_empty() && line != "\r\n" {
                            break;
                        }
                        self.buffer.append(&mut str_vec);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            self.buffer.reverse();
        }
        fn read_lines(&mut self) -> Result<String, &'static str> {
            let mut input = String::new();
            return match std::io::stdin().read_line(&mut input) {
                Ok(_) => Ok(input),
                Err(_) => Err("Reading failed"),
            };
        }
        pub fn gnn(&mut self) -> i32 {
            self.next::<i32>()
        }
        pub fn bgnn(&mut self) -> i64 {
            self.next::<i64>()
        }
        pub fn gs(&mut self) -> String {
            self.next::<String>()
        }
        pub fn gsb(&mut self) -> Vec<u8> {
            self.next::<String>().as_bytes().to_vec()
        }
        // pub fn gnn_vec(&mut self, n: i32) { (0..n).map(|_| self.gnn()) }
    }

    pub mod macros {
        macro_rules! abs {
            ($T:ty) => {
                pub fn abs<T>(a: T) -> T {
                    a
                }
            };
        }
        abs!(i32);
        // abs!(i128);
    }

    pub mod data_structure {}

    pub mod algorithm {}

    pub fn _binary_search_limit<'a>(
        v: &Vec<i64>,
        element: i64,
        space: usize,
    ) -> Result<(usize, usize), &'a str> {
        let mut low: usize = 0;
        if v.len() == 0 {
            return Err("Invalid vector.");
        }
        let mut high = v.len() - 1;
        let mut mid: usize;
        while (high - low) > space {
            mid = (high + low) / 2;
            if element <= v[mid] {
                high = mid;
            } else {
                low = mid;
            }
        }
        Ok((low, high))
    }

    pub fn binary_search(v: &Vec<i64>, element: i64) -> i64 {
        let (low, high) = _binary_search_limit(&v, element, 10).unwrap();
        for idx in low..(high + 1) {
            if element == v[idx] {
                return idx as i64;
            }
        }
        -1
    }

    pub fn parse_number(num: &String) -> i32 {
        num.parse::<i32>().unwrap()
    }
}

fn print<T: std::fmt::Debug>(x: &T) {
    println!("{:#?}", x);
}

// const INF: i64 = 0x3f3f3f3f;
const INF: i64 = 9_223_372_036_854_775_807;
const INF32: i32 = 2_147_483_647;

fn get_dir(ch: char) -> (i32, i32) {
    return match ch {
        'N' => (0, 1),
        'S' => (0, -1),
        'E' => (1, 0),
        'W' => (-1, 0),
        _ => (0, 0),
    };
}

macro_rules! read_array {
    ($cin:ident, $($t:ty),+) => {{
        (
            $($cin.next::<$t>(),)+
        )
    }};
}
fn binary_search(arr: &Vec<i32>, target: i32) -> Result<usize, usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        // Adjust bounds based on comparison
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if left == right && arr[left] == target {
        return Ok(left);
    }
    // If no match was found, return None
    Err(left)
}

fn main() {
    let mut cin = Scanner::new();

    let s = cin.gs();
    let p = cin.gs();
    println!("{}", is_match(s, p));

    // let t = cin.gnn();
    // for _ in 0..t {
    // solve_individual_case(&mut cin);
    // }
}

fn solve_individual_case(cin: &mut Scanner) {}

pub fn is_match(s: String, p: String) -> bool {
    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp[0][0] = true;
    for i in 0..=s.len() {
        for j in 1..=p.len() {
            if p.chars().nth(j - 1).unwrap() == '*' {
                dp[i][j] = dp[i][j - 1] || (i > 0 && dp[i - 1][j]);
            } else {
                dp[i][j] = i > 0
                    && dp[i - 1][j - 1]
                    && (s.chars().nth(i - 1).unwrap() == p.chars().nth(j - 1).unwrap()
                    || p.chars().nth(j - 1).unwrap() == '.');
            }
        }
    }
    print!("{}\t", ' ');
    for i in 1..=p.len() {
        print!("\t{}", p.chars().nth(i-1).unwrap());
    }
    println!();
    for i in 1..=s.len() {
        print!("{}", s.chars().nth(i-1).unwrap());
        for j in 0..=p.len() {
            print!("\t{}", dp[i][j]);
        }
        println!();
    }
    return true;
}
