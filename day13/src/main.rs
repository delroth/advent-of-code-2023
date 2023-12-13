use std::io::{self, Read};

fn is_symm_vert(patt: &Vec<&str>, x: usize, ndiff: u64) -> bool {
    let mut c1 = x - 1;
    let mut c2 = x;
    let mut ndiff = ndiff;

    loop {
        let diff = patt
            .iter()
            .map(|l| (l.chars().nth(c1).unwrap() != l.chars().nth(c2).unwrap()) as u64)
            .sum::<u64>();
        if diff > ndiff {
            return false;
        }
        ndiff -= diff;

        if c1 == 0 || c2 == patt[0].len() - 1 {
            if ndiff == 0 {
                return true;
            } else {
                return false;
            }
        }
        c1 -= 1;
        c2 += 1;
    }
}

fn is_symm_horiz(patt: &Vec<&str>, y: usize, ndiff: u64) -> bool {
    let mut r1 = y - 1;
    let mut r2 = y;
    let mut ndiff = ndiff;

    loop {
        let diff = (0..patt[0].len())
            .map(|i| (patt[r1].chars().nth(i).unwrap() != patt[r2].chars().nth(i).unwrap()) as u64)
            .sum::<u64>();
        if diff > ndiff {
            return false;
        }
        ndiff -= diff;

        if r1 == 0 || r2 == patt.len() - 1 {
            if ndiff == 0 {
                return true;
            } else {
                return false;
            }
        }
        r1 -= 1;
        r2 += 1;
    }
}

fn find_symm(patt: &Vec<&str>, ndiff: u64) -> u64 {
    let w = patt[0].len();
    for x in 1..w {
        if is_symm_vert(patt, x, ndiff) {
            return x as u64;
        }
    }
    for y in 1..patt.len() {
        if is_symm_horiz(patt, y, ndiff) {
            return (y as u64) * 100;
        }
    }
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let patterns: Vec<_> = input
        .trim()
        .split("\n\n")
        .map(|p| p.split("\n").collect::<Vec<_>>())
        .collect();

    let part1: u64 = patterns.iter().map(|l| find_symm(l, 0)).sum();
    println!("Part 1: {}", part1);

    let part2: u64 = patterns.iter().map(|l| find_symm(l, 1)).sum();
    println!("Part 2: {}", part2);
}
