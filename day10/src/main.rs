use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

type Pos = (i32, i32);
type Grid = HashMap<Pos, char>;
type DistMap = HashMap<Pos, u32>;

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
    let start: Pos = grid
        .iter()
        .filter(|&(_, &c)| c == 'S')
        .map(|(&p, _)| p)
        .next()
        .unwrap();

    let mut dists = DistMap::new();
    let mut queue = VecDeque::<Pos>::new();

    dists.insert(start, 0);
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        let dist = *dists.get(&pos).unwrap();

        let next: Vec<Pos> = match grid.get(&pos).unwrap() {
            // Manually hacked based on the input...
            'S' => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
            //'S' => vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)],
            '|' => vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1)],
            '-' => vec![(pos.0 - 1, pos.1), (pos.0 + 1, pos.1)],
            'L' => vec![(pos.0, pos.1 - 1), (pos.0 + 1, pos.1)],
            'J' => vec![(pos.0, pos.1 - 1), (pos.0 - 1, pos.1)],
            '7' => vec![(pos.0, pos.1 + 1), (pos.0 - 1, pos.1)],
            'F' => vec![(pos.0, pos.1 + 1), (pos.0 + 1, pos.1)],
            _ => vec![],
        };

        let next: Vec<Pos> = next
            .iter()
            .cloned()
            .filter(|p| grid.contains_key(p))
            .filter(|p| !dists.contains_key(p))
            .collect();
        for &n in next.iter() {
            queue.push_back(n);
            dists.insert(n, dist + 1);
        }
    }

    let dist: u32 = *dists.values().max().unwrap();
    println!("Dist: {}", dist);

    let mut area: u32 = 0;
    let maxy = grid.keys().map(|&(_, y)| y).max().unwrap();
    let maxx = grid.keys().map(|&(x, _)| x).max().unwrap();
    for y in 0..maxy {
        let mut inside = false;
        for x in 0..maxx {
            if dists.contains_key(&(x, y)) {
                let c = *grid.get(&(x, y)).unwrap();
                if c == '|' || c == 'F' || c == '7' {
                    inside = !inside;
                }
            } else if inside {
                area += 1;
            }
        }
    }

    println!("Area: {}", area);
}
