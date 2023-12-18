use std::collections::HashSet;
use std::io::{self, Read};

type Coord = (i64, i64);
type Grid = HashSet<Coord>;

type Segment = (Coord, Coord);
type Segments = Vec<Segment>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let insts: Vec<_> = input
        .trim()
        .split("\n")
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .collect();

    let mut grid = Grid::new();
    let (mut sx, mut sy) = (0, 0);
    for inst in insts.iter() {
        let dir = match inst[0] {
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            "U" => (0, -1),
            _ => unreachable!(),
        };
        let n = inst[1].parse::<u32>().unwrap();

        for _ in 0..n {
            grid.insert((sx, sy));
            sx += dir.0;
            sy += dir.1;
        }
    }

    let mut st = vec![(1, 1)];
    while let Some((x, y)) = st.pop() {
        if grid.contains(&(x, y)) {
            continue;
        }
        grid.insert((x, y));

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (x + dx, y + dy);
            if !grid.contains(&(nx, ny)) {
                st.push((nx, ny));
            }
        }
    }

    let part1 = grid.len();
    println!("Part 1: {}", part1);

    let mut points = Segments::new();
    let mut looplen = 0;
    let (mut sx, mut sy) = (0, 0);
    for inst in insts.iter() {
        let dir = match inst[2].chars().nth(7).unwrap() {
            '0' => (1, 0),
            '1' => (0, 1),
            '2' => (-1, 0),
            '3' => (0, -1),
            _ => unreachable!(),
        };
        let n = i64::from_str_radix(&inst[2][2..7], 16).unwrap();
        //let n = inst[1].parse::<i64>().unwrap();

        looplen += n;

        points.push(((sx, sy), (sx + dir.0 * n, sy + dir.1 * n)));
        sx += dir.0 * n;
        sy += dir.1 * n;
    }

    let mut dblarea: i64 = 0;
    for (a, b) in points.iter() {
        dblarea += a.0 * b.1 - a.1 * b.0;
    }
    dblarea /= 2;

    println!("Part 2: {}", dblarea + 1 + looplen / 2);
}
