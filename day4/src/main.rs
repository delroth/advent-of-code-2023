use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let cards: Vec<_> = input
        .trim()
        .split("\n")
        .map(|l| {
            l.split(":")
                .nth(1)
                .unwrap()
                .trim()
                .split(" | ")
                .map(|x| {
                    x.split(" ")
                        .map(|n| n.trim())
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<HashSet<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let sum1: u32 = cards
        .iter()
        .map(|p| (1 << (p[0].intersection(&p[1]).count() as u32)) >> 1)
        .sum();

    println!("Sum 1: {}", sum1);

    let mut sum2: u32 = 0;
    let mut stack = (0..cards.len()).collect::<Vec<_>>();
    while let Some(i) = stack.pop() {
        let n = cards[i][0].intersection(&cards[i][1]).count();
        for j in i + 1..i + 1 + n {
            if j >= cards.len() {
                break;
            }
            stack.push(j);
        }
        sum2 += 1
    }

    println!("Sum 2: {}", sum2);
}
