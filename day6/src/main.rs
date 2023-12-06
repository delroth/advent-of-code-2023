use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let lines = input.trim().split("\n").collect::<Vec<_>>();

    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let dists = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let sum1: u64 = times
        .iter()
        .enumerate()
        .map(|(i, &maxt)| {
            let dist = dists[i];
            let mut possible: u64 = 0;

            for hold in 0..maxt {
                let mut traveled: u64 = 0;
                let mut speed: u64 = 0;

                for t in 0..hold {
                    speed += 1;
                }

                for t in hold..maxt {
                    traveled += speed;
                }

                if traveled > dist {
                    possible += 1;
                }
            }

            possible
        })
        .product();

    println!("Sum 1: {}", sum1);
}
