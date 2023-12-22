use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

type Coord = (i32, i32, i32);
type Brick = (Coord, Coord);
type BrickId = usize;
type BrickMap = HashMap<BrickId, Brick>;
type BrickIdSet = HashSet<BrickId>;
type Space = HashMap<Coord, BrickId>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let bricks: Vec<Brick> = input
        .trim()
        .split("\n")
        .map(|l| {
            let mut p = l.split("~").map(|s| {
                let mut n = s.split(",").map(|i| i.parse::<i32>().unwrap());
                (n.next().unwrap(), n.next().unwrap(), n.next().unwrap())
            });
            (p.next().unwrap(), p.next().unwrap())
        })
        .collect();

    let brickmap: BrickMap = bricks.iter().enumerate().map(|(i, &x)| (i, x)).collect();

    let mut space = Space::new();
    for (&id, &((x1, y1, z1), (x2, y2, z2))) in brickmap.iter() {
        let (mut x, mut y, mut z) = (x1, y1, z1);
        let (dx, dy, dz) = (x2 - x1, y2 - y1, z2 - z1);
        while x != x2 || y != y2 || z != z2 {
            space.insert((x, y, z), id);
            x += dx.signum();
            y += dy.signum();
            z += dz.signum();
        }
        space.insert((x, y, z), id);
    }

    let mut settled = false;
    let mut safe_bids = BrickIdSet::new();
    let mut dependents = HashMap::<usize, Vec<usize>>::new();
    while !settled {
        settled = true;
        safe_bids.clear();
        dependents = brickmap.keys().map(|&i| (i, vec![])).collect();

        for &id in brickmap.keys() {
            let coords = || space.iter().filter(|(_, &i)| i == id).map(|(&c, _)| c);
            if coords().any(|(x, y, z)| z == 1) {
                continue;
            }
            let below: BrickIdSet = coords()
                .filter_map(|(x, y, z)| space.get(&(x, y, z - 1)))
                .map(|&i| i)
                .filter(|&i| i != id)
                .collect();
            if !below.is_empty() {
                if below.len() == 1 {
                    safe_bids.insert(*below.iter().next().unwrap());
                }
                for &b in below.iter() {
                    dependents.get_mut(&id).unwrap().push(b);
                }
                continue;
            }
            let cc = coords().collect::<Vec<Coord>>();
            for &(x, y, z) in cc.iter() {
                space.remove(&(x, y, z));
            }
            for &(x, y, z) in cc.iter() {
                space.insert((x, y, z - 1), id);
            }
            settled = false;
        }
    }

    println!("Part 1: {}", brickmap.len() - safe_bids.len());

    let mut tot_falls: usize = 0;
    for &id in brickmap.keys() {
        let mut falls = BrickIdSet::new();
        falls.insert(id);

        let mut settled = false;
        while !settled {
            settled = true;

            for &i in brickmap.keys() {
                if falls.contains(&i) {
                    continue;
                }

                let deps: BrickIdSet = dependents.get(&i).unwrap().iter().cloned().collect();
                if !deps.is_empty() && deps.iter().all(|&d| falls.contains(&d)) {
                    falls.insert(i);
                    settled = false;
                }
            }
        }

        tot_falls += falls.len() - 1;
    }
    println!("Part 2: {}", tot_falls);
}
