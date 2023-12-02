use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut avail = HashMap::<String, u32>::new();
    avail.insert("red".to_string(), 12);
    avail.insert("green".to_string(), 13);
    avail.insert("blue".to_string(), 14);

    let sum1: u32 = input
        .trim()
        .split("\n")
        .map(|l| {
            let parts: Vec<_> = l.split(' ').collect();
            let gid = parts[1].trim_end_matches(':').parse::<u32>().unwrap();

            let mut it = parts.iter().skip(2);
            while let Some(n) = it.next() {
                let n = n.parse::<u32>().unwrap();
                let col = it
                    .next()
                    .unwrap()
                    .trim_end_matches(|c| c == ',' || c == ';');

                if n > *avail.get(col).unwrap() {
                    return 0;
                }
            }

            gid
        })
        .sum();

    println!("Sum 1: {}", sum1);

    let sum2: u32 = input
        .trim()
        .split("\n")
        .map(|l| {
            let parts: Vec<_> = l.split(' ').collect();

            avail.insert("red".to_string(), 0);
            avail.insert("green".to_string(), 0);
            avail.insert("blue".to_string(), 0);

            let mut it = parts.iter().skip(2);
            while let Some(n) = it.next() {
                let n = n.parse::<u32>().unwrap();
                let col = it
                    .next()
                    .unwrap()
                    .trim_end_matches(|c| c == ',' || c == ';');

                if n > *avail.get(col).unwrap() {
                    avail.insert(col.to_string(), n);
                }
            }

            *avail.get("red").unwrap() * *avail.get("green").unwrap() * *avail.get("blue").unwrap()
        })
        .sum();

    println!("Sum 2: {}", sum2);
}
