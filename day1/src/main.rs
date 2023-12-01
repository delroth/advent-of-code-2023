use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let sum1: u32 = input
        .trim()
        .split("\n")
        .map(|l| {
            let leftmost = l
                .chars()
                .nth(l.find(|c: char| c.is_numeric()).unwrap())
                .unwrap();
            let rightmost = l
                .chars()
                .nth(l.rfind(|c: char| c.is_numeric()).unwrap())
                .unwrap();

            leftmost.to_digit(10).unwrap() * 10 + rightmost.to_digit(10).unwrap()
        })
        .sum();

    println!("Sum 1: {}", sum1);

    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let sum2: u32 = input
        .trim()
        .split("\n")
        .map(|l| {
            let mut li = l.find(|c: char| c.is_numeric()).unwrap();
            let mut ri = l.rfind(|c: char| c.is_numeric()).unwrap();

            let mut leftmost = l.chars().nth(li).unwrap().to_digit(10).unwrap();
            let mut rightmost = l.chars().nth(ri).unwrap().to_digit(10).unwrap();

            for (i, d) in DIGITS.iter().enumerate() {
                if let Some(li2) = l.find(d) {
                    if li2 <= li {
                        leftmost = (i as u32) + 1;
                        li = li2
                    }
                }
                if let Some(ri2) = l.rfind(d) {
                    if ri2 >= ri {
                        rightmost = (i as u32) + 1;
                        ri = ri2
                    }
                }
            }

            leftmost * 10 + rightmost
        })
        .sum();

    println!("Sum 2: {}", sum2);
}
