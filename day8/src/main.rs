use num::Integer;
use std::collections::HashMap;
use std::io::{self, Read};

type Map = HashMap<String, (String, String)>;

fn find_path_len(map: &Map, path: &String, from: String, to: String) -> usize {
    let mut res: usize = 0;
    let mut curr = from;

    while curr != to {
        let dir = path.chars().nth(res % path.len()).unwrap();
        let (l, r) = map.get(&curr).unwrap();
        if dir == 'L' {
            curr = l.to_string();
        } else {
            curr = r.to_string();
        }
        res += 1;
    }

    res
}

fn find_path_len_z(map: &Map, path: &String, from: String) -> usize {
    let mut res: usize = 0;
    let mut curr = from;

    while curr.chars().last().unwrap() != 'Z' {
        let dir = path.chars().nth(res % path.len()).unwrap();
        let (l, r) = map.get(&curr).unwrap();
        if dir == 'L' {
            curr = l.to_string();
        } else {
            curr = r.to_string();
        }
        res += 1;
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let path = input.trim().split("\n").next().unwrap().to_string();

    let map = input
        .trim()
        .split("\n")
        .skip(2)
        .map(|l| {
            let k = &l[0..3];
            let ll = &l[7..10];
            let lr = &l[12..15];

            (k.to_string(), (ll.to_string(), lr.to_string()))
        })
        .collect::<HashMap<_, _>>();

    let res1 = find_path_len(&map, &path, "AAA".to_string(), "ZZZ".to_string());
    println!("Result 1: {}", res1);

    let srcs = map
        .keys()
        .filter(|n| n.chars().nth(2).unwrap() == 'A')
        .collect::<Vec<_>>();
    let res2 = srcs
        .iter()
        .map(|s| find_path_len_z(&map, &path, s.to_string()))
        .fold(1usize, |acc, x| acc.lcm(&x));
    println!("Result 2: {}", res2);
}
