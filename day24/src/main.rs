use std::io::{self, Read};

type Coord = (f64, f64, f64);

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;
const MAX_T: f64 = MAX;

#[derive(Clone, Copy, Debug)]
struct Stone {
    p: Coord,
    v: Coord,
}

fn parse_coord(s: &str) -> Coord {
    let mut p = s.split(", ").map(|x| x.trim());
    (p.next().unwrap().parse::<f64>().unwrap(), p.next().unwrap().parse::<f64>().unwrap(), p.next().unwrap().parse::<f64>().unwrap())
}

fn intersects(s1: Stone, s2: Stone) -> bool {
    let b1 = s1.p;
    let b2 = s2.p;

    let e1 = (s1.p.0 + MAX_T * s1.v.0, s1.p.1 + MAX_T * s1.v.1, s1.p.2 + MAX_T * s1.v.2);
    let e2 = (s2.p.0 + MAX_T * s2.v.0, s2.p.1 + MAX_T * s2.v.1, s2.p.2 + MAX_T * s2.v.2);

    let (x1, x2, x3, x4) = (b1.0, e1.0, b2.0, e2.0);
    let (y1, y2, y3, y4) = (b1.1, e1.1, b2.1, e2.1);

    let den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    if den == 0.0 {
        return false;
    }

    let xi = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / den;
    let yi = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / den;

    return xi >= MIN && xi <= MAX && yi >= MIN && yi <= MAX && x1.min(x2) <= xi && x3.min(x4) <= xi && x1.max(x2) >= xi && x3.max(x4) >= xi;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let stones: Vec<Stone> = input.trim().split("\n").map(|l| { let mut p = l.split(" @ "); Stone{p: parse_coord(p.next().unwrap()), v: parse_coord(p.next().unwrap())} }).collect();

    let mut count: u32 = 0;
    for i in 0..stones.len() {
        for j in i+1..stones.len() {
            if intersects(stones[i], stones[j]) {
                count += 1;
            }
        }
    }

    println!("Part 1: {}", count);
}
