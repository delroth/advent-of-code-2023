use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::io::{self, Read};

type Coord = (i32, i32);
type Grid = HashMap<Coord, u32>;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct State {
    pos: Coord,
    dir: Coord,
    len: u32,
}

impl Ord for State {
    fn cmp(&self, o: &Self) -> Ordering {
        o.len
            .cmp(&self.len)
            .then_with(|| self.pos.cmp(&o.pos))
            .then_with(|| self.dir.cmp(&o.dir))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

fn solve(grid: &Grid, min_straight: u32, max_straight: u32) -> u32 {
    let w = grid.keys().map(|(x, _)| x).max().unwrap() + 1;
    let h = grid.keys().map(|(_, y)| y).max().unwrap() + 1;

    let mut q = BinaryHeap::<State>::new();
    let mut explored = HashMap::<State, u32>::new();
    let mut len_found: u32 = 0;

    q.push(State {
        pos: (0, 0),
        dir: (0, 0),
        len: 0,
    });

    while let Some(st) = q.pop() {
        if st.pos == (w - 1, h - 1) {
            len_found = st.len;
            break;
        }

        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            if (dx, dy) == st.dir || (dx, dy) == (-st.dir.0, -st.dir.1) {
                continue;
            }

            let (mut cx, mut cy) = st.pos;
            let mut cost_sum = 0;
            for s in 0..max_straight {
                cx += dx;
                cy += dy;

                let cost = grid.get(&(cx, cy));
                if cost == None {
                    break;
                }
                cost_sum += *cost.unwrap();

                if (s + 1) < min_straight {
                    continue;
                }

                let nlen = st.len + cost_sum;
                let mut nst = State {
                    pos: (cx, cy),
                    dir: (dx, dy),
                    len: 0,
                };
                if *explored.get(&nst).unwrap_or(&1000000) <= nlen {
                    continue;
                }
                explored.insert(nst, nlen);
                nst.len = nlen;
                q.push(nst);
            }
        }
    }

    len_found
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
                .map(move |(x, c)| ((x as i32, y as i32), c.to_digit(10).unwrap()))
        })
        .collect();

    println!("Part 1: {}", solve(&grid, 0, 3));

    println!("Part 2: {}", solve(&grid, 4, 10));
}
