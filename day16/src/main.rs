use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Coord = (i32, i32);
type Grid = HashMap<Coord, char>;

fn get_energized(grid: &Grid, st: Coord, sd: Coord) -> usize {
    let mut energized = HashSet::<Coord>::new();
    let mut heads = Vec::<(Coord, Coord)>::new();
    let mut explored = HashSet::<(Coord, Coord)>::new();

    heads.push((st, sd));

    while let Some(((x, y), (dx, dy))) = heads.pop() {
        if !grid.contains_key(&(x, y)) {
            continue;
        }

        energized.insert((x, y));

        let c = *grid.get(&(x, y)).unwrap();
        let mut next = vec![];

        if c == '.' {
            next.push((dx, dy));
        } else if c == '/' {
            next.push((-dy, -dx))
        } else if c == '\\' {
            next.push((dy, dx))
        } else if c == '|' {
            if dx == 0 {
                next.push((dx, dy));
            } else {
                next.push((0, -1));
                next.push((0, 1));
            }
        } else if c == '-' {
            if dy == 0 {
                next.push((dx, dy));
            } else {
                next.push((-1, 0));
                next.push((1, 0));
            }
        }

        for &(dx, dy) in next.iter() {
            let (nx, ny) = (x + dx, y + dy);
            if !explored.contains(&((nx, ny), (dx, dy))) {
                heads.push(((nx, ny), (dx, dy)));
                explored.insert(((nx, ny), (dx, dy)));
            }
        }
    }

    energized.len()
}

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

    let part1 = get_energized(&grid, (0, 0), (1, 0));
    println!("Part 1: {}", part1);

    let w = grid.keys().map(|(x, _)| x).max().unwrap() + 1;
    let h = grid.keys().map(|(x, _)| x).max().unwrap() + 1;
    let mut all_pos: Vec<(Coord, Coord)> = vec![];
    for x in 0..w {
        all_pos.push(((x, 0), (0, 1)));
        all_pos.push(((x, h - 1), (0, -1)));
    }
    for y in 0..h {
        all_pos.push(((0, y), (1, 0)));
        all_pos.push(((w - 1, y), (-1, 0)));
    }
    let part2 = all_pos
        .iter()
        .map(|&(p, d)| get_energized(&grid, p, d))
        .max()
        .unwrap();
    println!("Part 2: {}", part2);
}
