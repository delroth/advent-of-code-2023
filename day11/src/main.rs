use std::collections::HashMap;
use std::io::{self, Read};

type Pos = (i32, i32);
type Grid = HashMap<Pos, char>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let grid: Grid = input
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    let h = grid.keys().map(|&(_, y)| y).max().unwrap() + 1;
    let w = grid.keys().map(|&(x, _)| x).max().unwrap() + 1;

    let dbl_r: Vec<i32> = (0..h)
        .filter(|&y| (0..w).all(|x| *grid.get(&(x, y)).unwrap() == '.'))
        .collect();
    let dbl_c: Vec<i32> = (0..w)
        .filter(|&x| (0..h).all(|y| *grid.get(&(x, y)).unwrap() == '.'))
        .collect();

    let part1: i32 = grid
        .iter()
        .filter(|&(_, &c)| c == '#')
        .map(|(&p1, _)| {
            grid.iter()
                .filter(move |&(&p, &c)| p < p1 && c == '#')
                .map(|(&p2, _)| {
                    let mut dx: i32 = (p1.0 - p2.0).abs();
                    let mut dy: i32 = (p1.1 - p2.1).abs();
                    for &dbl in dbl_c.iter() {
                        if p1.0.min(p2.0) < dbl && dbl < p1.0.max(p2.0) {
                            dx += 1
                        }
                    }
                    for &dbl in dbl_r.iter() {
                        if p1.1.min(p2.1) < dbl && dbl < p1.1.max(p2.1) {
                            dy += 1
                        }
                    }
                    dx + dy
                })
                .sum::<i32>()
        })
        .sum();
    println!("Part 1: {}", part1);

    let part2: i64 = grid
        .iter()
        .filter(|&(_, &c)| c == '#')
        .map(|(&p1, _)| {
            grid.iter()
                .filter(move |&(&p, &c)| p < p1 && c == '#')
                .map(|(&p2, _)| {
                    let mut dx: i64 = (p1.0 - p2.0).abs() as i64;
                    let mut dy: i64 = (p1.1 - p2.1).abs() as i64;
                    for &dbl in dbl_c.iter() {
                        if p1.0.min(p2.0) < dbl && dbl < p1.0.max(p2.0) {
                            dx += 999999
                        }
                    }
                    for &dbl in dbl_r.iter() {
                        if p1.1.min(p2.1) < dbl && dbl < p1.1.max(p2.1) {
                            dy += 999999
                        }
                    }
                    dx + dy
                })
                .sum::<i64>()
        })
        .sum();
    println!("Part 2: {}", part2);
}
