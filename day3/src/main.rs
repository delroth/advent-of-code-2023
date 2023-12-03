use multimap::MultiMap;
use std::collections::HashMap;
use std::io::{self, Read};

type Coord = (i32, i32);
type Grid = HashMap<Coord, char>;
type Gears = MultiMap<Coord, u32>;

fn consume_number(g: &Grid, x: &mut i32, y: i32) -> u32 {
    let mut n = 0;
    while let Some(&c) = g.get(&(*x, y)) {
        if c.is_numeric() {
            *x += 1;
            n *= 10;
            n += c.to_digit(10).unwrap();
        } else {
            break;
        }
    }
    n
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
                .map(|(x, c)| ((x as i32, y as i32), c))
                .collect::<Vec<_>>()
        })
        .collect();

    let mut gears = Gears::new();

    let w = grid.iter().map(|(&(x, _), _)| x).max().unwrap() + 1;
    let h = grid.iter().map(|(&(_, y), _)| y).max().unwrap() + 1;

    let mut sum1: u32 = 0;
    for y in 0..h {
        let mut x = 0;

        while x < w {
            let ox = x;
            let c = *grid.get(&(x, y)).unwrap();
            if !c.is_numeric() {
                x += 1;
            } else {
                let n = consume_number(&grid, &mut x, y);
                let mut adjacent = false;
                'test: for tx in ox..x {
                    for (dx, dy) in [
                        (-1, 0),
                        (1, 0),
                        (0, -1),
                        (0, 1),
                        (-1, -1),
                        (-1, 1),
                        (1, -1),
                        (1, 1),
                    ]
                    .iter()
                    {
                        if let Some(&c) = grid.get(&(tx + dx, y + dy)) {
                            if !c.is_numeric() && c != '.' {
                                adjacent = true;
                                if c == '*' {
                                    gears.insert((tx + dx, y + dy), n);
                                }
                                break 'test;
                            }
                        }
                    }
                }
                if adjacent {
                    sum1 += n;
                }
            }
        }
    }

    println!("Sum 1: {}", sum1);

    let sum2: u32 = gears
        .iter_all()
        .map(|(_, vals)| vals)
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum();
    println!("Sum 2: {}", sum2);
}
