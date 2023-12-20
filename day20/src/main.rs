use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug, Eq, PartialEq)]
enum GateType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

#[derive(Debug, Eq, PartialEq)]
struct Gate {
    typ: GateType,
    dst: Vec<String>,
    inp: Vec<String>,
    high: bool,
    on: bool,
    first_on: u32,
    last_on: u32,
}

type Circuit = HashMap<String, Gate>;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut revmap = HashMap::<String, Vec<String>>::new();
    let mut circuit: Circuit = input
        .trim()
        .split("\n")
        .map(|l| {
            let mut parts = l.split(" -> ");
            let name = parts.next().unwrap();
            let dst: Vec<String> = parts
                .next()
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect();

            let typ = match name.chars().nth(0).unwrap() {
                '%' => GateType::FlipFlop,
                '&' => GateType::Conjunction,
                _ => GateType::Broadcaster,
            };

            let name = if typ == GateType::Broadcaster {
                name.to_string()
            } else {
                name[1..name.len()].to_string()
            };

            for d in dst.iter() {
                revmap.entry(d.clone()).or_insert(vec![]).push(name.clone());
            }

            (
                name,
                Gate {
                    typ,
                    dst,
                    inp: vec![],
                    high: false,
                    on: false,
                    first_on: 0,
                    last_on: 0,
                },
            )
        })
        .collect();

    for (n, inp) in revmap.iter() {
        if let Some(g) = circuit.get_mut(n) {
            g.inp = inp.clone();
        }
    }

    let mut lowcnt: u32 = 0;
    let mut highcnt: u32 = 0;

    for i in 0..100000000 {
        let mut signals = vec![("broadcaster".to_string(), false)];

        while !signals.is_empty() {
            let mut new_signals = vec![];

            for (name, high) in signals.iter() {
                if *high {
                    highcnt += 1;
                } else {
                    lowcnt += 1;
                }

                let gate = circuit.get(name);
                if gate == None {
                    if !*high {
                        println!("Found answer: {}", i + 1);
                    }
                    continue;
                }
                let inp_all_high = gate
                    .unwrap()
                    .inp
                    .iter()
                    .all(|i| circuit.get(&i.clone()).unwrap().high);
                let gate = circuit.get_mut(name).unwrap();

                match gate.typ {
                    GateType::Broadcaster => {
                        gate.high = *high;
                        for dst in gate.dst.iter() {
                            new_signals.push((dst.clone(), *high));
                        }
                    }
                    GateType::FlipFlop => {
                        if !*high {
                            gate.on = !gate.on;
                            gate.high = gate.on;
                            for dst in gate.dst.iter() {
                                new_signals.push((dst.clone(), gate.on));
                            }
                        }
                    }
                    GateType::Conjunction => {
                        let out = !inp_all_high;
                        gate.high = out;

                        if !gate.high {
                            if gate.first_on == 0 {
                                gate.first_on = i;
                            } else if gate.last_on == 0 {
                                gate.last_on = i;
                            } else {
                                if i - gate.last_on > 1000 {
                                    println!(
                                        "Conj {}: first on {}, cycle: {}",
                                        name,
                                        gate.first_on + 1,
                                        i - gate.last_on
                                    );
                                }
                                gate.last_on = i;
                            }
                        }
                        for dst in gate.dst.iter() {
                            new_signals.push((dst.clone(), out));
                        }
                    }
                }
            }

            signals = new_signals;
        }
    }

    println!("Part 1: {}", lowcnt * highcnt);

    // Solved by looking at the conj cycles manually in the output then computing the LCM of those
    // cycles to figure out when they'd all be low together.
    let part2: u32 = 0;
    println!("Part 2: {}", part2);
}
