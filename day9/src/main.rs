use std::io::{self, Read};

struct Series {
    vals: Vec<i32>,
    pred: Vec<Vec<i32>>,
}

fn make_pred(vals: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![vals.clone()];
    let mut last = &res[0];

    while last.iter().filter(|&&n| n != 0).count() != 0 {
        let newl = last
            .iter()
            .zip(last.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        res.push(newl);
        last = &res[res.len() - 1];
    }

    res
}

impl Series {
    fn new(s: &str) -> Series {
        let vals = s.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        Series {
            pred: make_pred(&vals),
            vals,
        }
    }

    fn next(&self) -> i32 {
        self.pred.iter().map(|v| v.iter().last().unwrap()).sum()
    }

    fn prev(&self) -> i32 {
        self.pred
            .iter()
            .enumerate()
            .map(|(i, v)| v[0] * (if i % 2 == 0 { 1 } else { -1 }))
            .sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let series = input
        .trim()
        .split("\n")
        .map(|s| Series::new(s))
        .collect::<Vec<_>>();

    let sum1: i32 = series.iter().map(|s| s.next()).sum();
    println!("Sum 1: {}", sum1);

    let sum2: i32 = series.iter().map(|s| s.prev()).sum();
    println!("Sum 2: {}", sum2);
}
