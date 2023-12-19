use std::collections::HashMap;
use std::io::{self, Read};

type Workflow = Vec<Rule>;
type Workflows = HashMap<String, Workflow>;
type Part = HashMap<String, u32>;

type PartRanges = Vec<HashMap<String, (u32, u32)>>;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Rule {
    cond: Condition,
    act: Action,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Condition {
    Always,
    Lt(String, u32),
    Gt(String, u32),
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Action {
    Jump(String),
    Accept,
    Reject,
}

fn part_is_accepted(wfs: &Workflows, part: &Part) -> bool {
    let mut curr = String::from("in");

    loop {
        let wf = wfs.get(&curr).unwrap();

        for r in wf.iter() {
            let applies = match &r.cond {
                Condition::Always => true,
                Condition::Lt(vn, val) => part.get(vn).unwrap() < val,
                Condition::Gt(vn, val) => part.get(vn).unwrap() > val,
            };

            if !applies {
                continue;
            }

            match &r.act {
                Action::Accept => {
                    return true;
                }
                Action::Reject => {
                    return false;
                }
                Action::Jump(rn) => {
                    curr = rn.clone();
                    break;
                }
            }
        }
    }
}

fn max_part_ranges() -> PartRanges {
    let mut ret = HashMap::<String, (u32, u32)>::new();
    for v in ["x", "m", "a", "s"] {
        ret.insert(v.to_string(), (1, 4000));
    }
    return vec![ret];
}

fn concat_ranges(mut r1: PartRanges, mut r2: PartRanges) -> PartRanges {
    r1.append(&mut r2);
    r1
}

fn attenuate(p: PartRanges, cond: &Condition) -> PartRanges {
    match cond {
        Condition::Always => p,
        Condition::Lt(vn, val) => {
            let mut new_ranges = vec![];
            for vr in p.iter() {
                let mut vr = vr.clone();
                let &(b, e) = vr.get(vn).unwrap();
                if e < *val {
                    new_ranges.push(vr);
                } else if b < *val {
                    vr.insert(vn.clone(), (b, *val - 1));
                    new_ranges.push(vr);
                }
            }
            new_ranges
        }
        Condition::Gt(vn, val) => {
            let mut new_ranges = vec![];
            for vr in p.iter() {
                let mut vr = vr.clone();
                let &(b, e) = vr.get(vn).unwrap();
                if b > *val {
                    new_ranges.push(vr);
                } else if e > *val {
                    vr.insert(vn.clone(), (*val + 1, e));
                    new_ranges.push(vr);
                }
            }
            new_ranges
        }
    }
}

fn accepted_ranges(wfs: &Workflows, rn: &str, ri: usize) -> PartRanges {
    let rule = &wfs.get(rn).unwrap()[ri];

    let accepted_if_true = match &rule.act {
        Action::Accept => max_part_ranges(),
        Action::Reject => PartRanges::new(),
        Action::Jump(rn) => accepted_ranges(wfs, rn.as_str(), 0),
    };

    if rule.cond == Condition::Always {
        return accepted_if_true;
    }

    let accepted_if_false = accepted_ranges(wfs, rn, ri + 1);

    let inverse_cond = match &rule.cond {
        Condition::Lt(vn, val) => Condition::Gt(vn.to_string(), val - 1),
        Condition::Gt(vn, val) => Condition::Lt(vn.to_string(), val + 1),
        _ => unreachable!(),
    };

    let accepted_if_true = attenuate(accepted_if_true, &rule.cond);
    let accepted_if_false = attenuate(accepted_if_false, &inverse_cond);

    concat_ranges(accepted_if_true, accepted_if_false)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let workflows = input.trim().split("\n\n").nth(0).unwrap();
    let parts = input.trim().split("\n\n").nth(1).unwrap();

    let workflows: Workflows = workflows
        .split("\n")
        .map(|l| {
            let name = l.split("{").nth(0).unwrap();
            let conds = l.split("{").nth(1).unwrap().split("}").nth(0).unwrap();

            let conds: Workflow = conds
                .split(",")
                .map(|c| {
                    let act = c.split(":").last().unwrap().to_string();
                    let cond = c.split(":").nth(0).unwrap();

                    let cond = if act != cond {
                        let var = cond[0..1].to_string();
                        let val = cond[2..].parse::<u32>().unwrap();
                        match cond.chars().nth(1).unwrap() {
                            '<' => Condition::Lt(var, val),
                            '>' => Condition::Gt(var, val),
                            _ => unreachable!(),
                        }
                    } else {
                        Condition::Always
                    };

                    let act = match act.as_str() {
                        "R" => Action::Reject,
                        "A" => Action::Accept,
                        _ => Action::Jump(act),
                    };

                    Rule { cond, act }
                })
                .collect();

            (name.to_string(), conds)
        })
        .collect();

    let parts: Vec<Part> = parts
        .split("\n")
        .map(|l| {
            let l = &l[1..l.len() - 1];
            l.split(",")
                .map(|v| {
                    let name = v.split("=").nth(0).unwrap();
                    let val = v.split("=").nth(1).unwrap().parse::<u32>().unwrap();

                    (name.to_string(), val)
                })
                .collect()
        })
        .collect();

    let part1: u32 = parts
        .iter()
        .filter(|&p| part_is_accepted(&workflows, &p))
        .map(|p| p.values().sum::<u32>())
        .sum();
    println!("Part 1: {}", part1);

    let part2: u64 = accepted_ranges(&workflows, "in", 0)
        .iter()
        .map(|rs| {
            rs.values()
                .map(|(b, e)| ((e - b) + 1) as u64)
                .product::<u64>()
        })
        .sum();
    println!("Part 2: {}", part2);
}
