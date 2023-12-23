use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

type Coord = (i32, i32);
type Grid = HashMap<Coord, char>;
type GridLen = HashMap<Coord, u32>;

#[derive(Debug)]
struct State {
    curr: Coord,
    prev: Coord,
    len: u32,
    path: HashSet<Coord>,
}

fn part2(grid: &Grid, (x, y): Coord, (ex, ey): Coord, path: &mut HashSet<Coord>) -> usize {
    if (x, y) == (ex, ey) {
        return path.len();
    }
    if path.contains(&(x, y)) {
        return 0;
    }
    let c = *grid.get(&(x, y)).unwrap_or(&'#');
    if c == '#' {
        return 0;
    }

    path.insert((x, y));
    let l = *[(-1, 0), (1, 0), (0, -1), (0, 1)]
        .map(|(dx, dy)| part2(grid, (x + dx, y + dy), (ex, ey), path))
        .iter()
        .max()
        .unwrap();
    path.remove(&(x, y));
    return l;
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
    let h = grid.keys().map(|&(_, y)| y).max().unwrap() + 1;

    let (&(sx, sy), _) = grid
        .iter()
        .filter(|&(&(_, y), &c)| y == 0 && c == '.')
        .next()
        .unwrap();
    let (&(ex, ey), _) = grid
        .iter()
        .filter(|&(&(_, y), &c)| y == (h - 1) && c == '.')
        .next()
        .unwrap();

    let mut lengths = GridLen::new();
    let mut q = VecDeque::<State>::new();
    q.push_back(State {
        curr: (sx, sy),
        prev: (sx, sy - 1),
        len: 0,
        path: HashSet::<Coord>::new(),
    });

    while let Some(st) = q.pop_front() {
        lengths.insert(st.curr, st.len);

        for (arr, dx, dy) in [('<', -1, 0), ('>', 1, 0), ('^', 0, -1), ('v', 0, 1)] {
            let (nx, ny) = (st.curr.0 + dx, st.curr.1 + dy);
            if (nx, ny) == st.prev {
                continue;
            }
            let c = *grid.get(&(nx, ny)).unwrap_or(&'#');
            if c == '.' {
                q.push_back(State {
                    curr: (nx, ny),
                    prev: st.curr,
                    len: st.len + 1,
                    path: st.path.clone(),
                });
            } else if c == arr {
                q.push_back(State {
                    curr: (nx + dx, ny + dy),
                    prev: (nx, ny),
                    len: st.len + 2,
                    path: st.path.clone(),
                });
            }
        }
    }

    println!("Part 1: {}", lengths.get(&(ex, ey)).unwrap());

    println!(
        "Part 2: {}",
        part2(&grid, (sx, sy), (ex, ey), &mut HashSet::<Coord>::new())
    );
}
