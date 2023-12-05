use std::io::{self, Read};

struct Range {
    src: u64,
    len: u64,
    dst: u64,
}

impl Range {
    fn new(l: &str) -> Range {
        let mut p = l.split(" ").map(|n| n.parse::<u64>().unwrap());
        let dst = p.next().unwrap();
        let src = p.next().unwrap();
        let len = p.next().unwrap();

        Range { src, len, dst }
    }

    fn convert(&self, n: u64) -> u64 {
        if n >= self.src && n < self.src + self.len {
            self.dst + (n - self.src)
        } else {
            n
        }
    }

    fn split_input_range(&self, st: u64, cnt: u64) -> Vec<(u64, u64)> {
        let count_before = if st >= self.src {
            0
        } else {
            (self.src - st).min(cnt)
        };
        let count_after = if st + cnt < self.src + self.len {
            0
        } else {
            (st + cnt - self.src - self.len).min(cnt)
        };
        let count_in = cnt - count_before - count_after;

        assert!(count_before + count_in + count_after == cnt);

        let mut ret = vec![];
        if count_before > 0 {
            ret.push((st, count_before));
        }
        if count_in > 0 {
            ret.push((st + count_before, count_in))
        }
        if count_after > 0 {
            ret.push((st + count_before + count_in, count_after))
        }

        ret
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let sections = input.trim().split("\n\n").collect::<Vec<_>>();

    let seeds = sections[0]
        .split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let maps = sections
        .iter()
        .skip(1)
        .map(|s| s.split("\n").skip(1).collect::<Vec<_>>())
        .map(|s| s.iter().map(|r| Range::new(r)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let locs = seeds
        .iter()
        .map(|&s| {
            let mut s = s;
            for map in maps.iter() {
                for r in map.iter() {
                    if r.convert(s) != s {
                        s = r.convert(s);
                        break;
                    }
                }
            }
            s
        })
        .collect::<Vec<_>>();

    let r1: u64 = *locs.iter().min().unwrap();

    println!("Result 1: {}", r1);

    let mut seeds = seeds.chunks(2).map(|p| (p[0], p[1])).collect::<Vec<_>>();
    for map in maps.iter() {
        let mut split_ranges = vec![];
        for &seed in seeds.iter() {
            let mut acc = vec![seed];
            for r in map.iter() {
                let mut new_acc = vec![];
                for &(st, cnt) in acc.iter() {
                    let mut split = r.split_input_range(st, cnt);
                    new_acc.append(&mut split);
                }
                acc = new_acc;
            }
            split_ranges.append(&mut acc);
        }

        let mut mapped_ranges = vec![];
        for &(st, cnt) in split_ranges.iter() {
            let mut mapped = false;
            for r in map.iter() {
                if st >= r.src && st < r.src + r.len {
                    assert!(st + cnt <= r.src + r.len);
                    mapped_ranges.push((r.convert(st), cnt));
                    mapped = true;
                    break;
                }
            }
            if !mapped {
                mapped_ranges.push((st, cnt))
            }
        }

        seeds = mapped_ranges;
    }

    let r2: u64 = seeds.iter().map(|&(st, _)| st).min().unwrap();

    println!("Result 2: {}", r2);
}
