use std::io::{self, Read};

#[derive(Copy, Clone)]
enum Op {
    Remove,
    Set(u32),
}

fn hash(s: &str) -> u32 {
    let mut curr: u32 = 0;

    for c in s.chars() {
        let c = c as u32;
        curr += c;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let parts: Vec<_> = input.trim().split(",").collect();

    let part1: u32 = parts.iter().map(|&s| hash(s)).sum();
    println!("Part 1: {}", part1);

    let ops: Vec<_> = parts
        .iter()
        .map(|&s| {
            if s.contains('-') {
                (s.split('-').next().unwrap(), Op::Remove)
            } else {
                (
                    s.split('=').next().unwrap(),
                    Op::Set(s.split('=').nth(1).unwrap().parse::<u32>().unwrap()),
                )
            }
        })
        .collect();
    let mut hashmap: Vec<Vec<(String, u32)>> = vec![];
    for i in 0..256 {
        hashmap.push(vec![]);
    }
    for &(l, op) in ops.iter() {
        let h = hash(l) as usize;
        match op {
            Op::Remove => {
                hashmap[h].retain(|x| x.0 != l);
            }
            Op::Set(x) => {
                let mut b = &mut hashmap[h];
                let mut found = false;
                for j in 0..b.len() {
                    if b[j].0 == l {
                        b[j] = (l.to_string(), x);
                        found = true;
                        break;
                    }
                }
                if !found {
                    b.push((l.to_string(), x));
                }
            }
        };
    }
    let part2: usize = hashmap
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, (_, l))| (i + 1) * (j + 1) * *l as usize)
                .sum::<usize>()
        })
        .sum();
    println!("Part 2: {}", part2);
}
