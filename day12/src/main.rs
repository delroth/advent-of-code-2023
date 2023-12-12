use cached::proc_macro::cached;
use itertools::Itertools;
use std::io::{self, Read};

fn num_solutions(conds: Vec<char>, counts: Vec<usize>) -> u64 {
    let mut possible_cnt: u64 = 0;

    let tot_oper: usize = conds.len() - counts.iter().sum::<usize>();
    for perms in (0..=tot_oper).combinations(counts.len()) {
        let mut expected = vec![];
        for i in 0..perms.len() {
            let prev = if i == 0 { 0 } else { perms[i - 1] };
            for _ in prev..perms[i] {
                expected.push('.');
            }
            for _ in 0..counts[i] {
                expected.push('#');
            }
        }
        while expected.len() < conds.len() {
            expected.push('.');
        }
        let mut possible = true;
        for i in 0..conds.len() {
            let c = conds[i];
            if c != '?' && c != expected[i] {
                possible = false;
                break;
            }
        }

        if possible {
            possible_cnt += 1
        }
    }
    possible_cnt
}

#[cached]
fn num_solutions2(conds: Vec<char>, counts: Vec<usize>) -> u64 {
    if counts.is_empty() {
        if conds.iter().all(|&c| c == '.' || c == '?') {
            1
        } else {
            0
        }
    } else {
        let mut solutions = 0;

        let g = counts[0];
        let min_rem = (counts.len() - 1) + counts.iter().sum::<usize>() - g;
        let max_oper_before = conds.len() - min_rem - g;
        for oper_before in 0..=max_oper_before {
            if (0..oper_before).any(|i| conds[i] != '.' && conds[i] != '?') {
                continue;
            }
            if (oper_before..oper_before + g).any(|i| conds[i] != '#' && conds[i] != '?') {
                continue;
            }
            if oper_before + g != conds.len()
                && conds[oper_before + g] != '.'
                && conds[oper_before + g] != '?'
            {
                continue;
            }

            let new_conds: Vec<_> = conds.iter().cloned().skip(oper_before + g + 1).collect();
            let new_counts: Vec<_> = counts.iter().cloned().skip(1).collect();
            solutions += num_solutions2(new_conds, new_counts)
        }

        solutions
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let problems: Vec<_> = input
        .trim()
        .split("\n")
        .map(|l| {
            let conditions: Vec<char> = l.split(" ").nth(0).unwrap().chars().collect();
            let counts: Vec<usize> = l
                .split(" ")
                .nth(1)
                .unwrap()
                .split(",")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            (conditions, counts)
        })
        .collect();

    let part1: u64 = problems
        .iter()
        .map(|(conds, counts)| num_solutions2(conds.clone(), counts.clone()))
        .sum();
    println!("Part 1: {}", part1);

    let part2: u64 = problems
        .iter()
        .map(|(conds, counts)| {
            let conds5: Vec<_> = (0..5)
                .flat_map(|i| {
                    let mut a = conds.clone();
                    if i != 4 {
                        a.push('?');
                    }
                    a
                })
                .collect();
            let counts5: Vec<_> = (0..5).flat_map(|_| counts.clone()).collect();
            num_solutions2(conds5, counts5)
        })
        .sum();
    println!("Part 2: {}", part2);
}
