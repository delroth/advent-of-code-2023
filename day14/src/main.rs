use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Read};

type Coord = (i32, i32);
type Grid = HashMap<Coord, char>;
type Seen = HashMap<String, u32>;
type Scores = HashMap<u32, i32>;

fn tilt(grid: &mut Grid) -> bool {
    let mut moved = false;

    let w = grid.keys().map(|&t| t.0).max().unwrap() + 1;
    let h = grid.keys().map(|&t| t.1).max().unwrap() + 1;

    for y in 0..h - 1 {
        for x in 0..w {
            let &c = grid.get(&(x, y)).unwrap();
            if c != '.' {
                continue;
            }
            let &c = grid.get(&(x, y + 1)).unwrap();
            if c == 'O' {
                grid.insert((x, y), 'O');
                grid.insert((x, y + 1), '.');
                moved = true;
            }
        }
    }

    moved
}

fn rot(grid: &Grid) -> Grid {
    let h = grid.keys().map(|&t| t.1).max().unwrap() + 1;

    grid.iter()
        .map(|(&(x, y), &c)| ((h - y - 1, x), c))
        .collect()
}

fn parse_grid(input: &String) -> Grid {
    input
        .trim()
        .split("\n")
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut grid = parse_grid(&input);
    while tilt(&mut grid) {}
    let h = grid.keys().map(|&t| t.1).max().unwrap() + 1;
    let part1: i32 = grid
        .iter()
        .map(|(&(_, y), &c)| if c != 'O' { 0 } else { h - y })
        .sum();
    println!("Part 1: {}", part1);

    let mut seen = Seen::new();
    let mut scores = Scores::new();
    let mut grid = parse_grid(&input);
    let mut i = 0;
    let mut period = 0;
    loop {
        let s: String = grid.iter().sorted().map(|(_, &c)| c).collect();
        let score: i32 = grid
            .iter()
            .map(|(&(_, y), &c)| if c != 'O' { 0 } else { h - y })
            .sum();
        if !seen.contains_key(&s) {
            seen.insert(s, i);
            scores.insert(i, score);
        } else {
            period = i - seen.get(&s).unwrap();
            break;
        }

        for _ in 0..4 {
            while tilt(&mut grid) {}
            grid = rot(&grid);
        }

        i += 1;
    }

    let off = i - period;
    let pi = off + (1000000000 - off) % period;
    let part2 = scores.get(&pi).unwrap();
    println!("Part 2: {}", part2);
}
