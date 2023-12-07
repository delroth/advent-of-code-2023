use std::collections::HashMap;
use std::io::{self, Read};

fn card_to_score(card: char) -> u32 {
    match card {
        'J' => 0,
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'Q' => 10,
        'K' => 11,
        'A' => 12,
        _ => panic!("wrong card"),
    }
}

fn hand_score(hand: &str, card_score: u32) -> u32 {
    let mut hmap = HashMap::new();

    for c in hand.chars() {
        hmap.insert(c, hmap.get(&c).unwrap_or(&0) + 1);
    }

    let mut freq = hmap.iter().map(|(&k, &c)| (k, c)).collect::<Vec<_>>();
    freq.sort_by_key(|&(a, c)| -(c as i32 * 13 + card_to_score(a) as i32));

    let score = if freq[0].1 == 5 {
        6000000 + card_score
    } else if freq[0].1 == 4 {
        5000000 + card_score
    } else if freq[0].1 == 3 && freq[1].1 == 2 {
        4000000 + card_score
    } else if freq[0].1 == 3 {
        3000000 + card_score
    } else if freq[0].1 == 2 && freq[1].1 == 2 {
        2000000 + card_score
    } else if freq[0].1 == 2 {
        1000000 + card_score
    } else {
        card_score
    };

    score
}

fn hand_score_joker(hand: &str, card_score: Option<u32>) -> u32 {
    let card_score = card_score.unwrap_or(
        hand.chars()
            .enumerate()
            .map(|(i, c)| card_to_score(c) * 13_u32.pow(4 - i as u32))
            .sum(),
    );
    let mut max_score = hand_score(hand, card_score);

    let mut chars = hand.chars().collect::<Vec<_>>();
    for i in 0..5 {
        if chars[i] == 'J' {
            for &c in ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'].iter() {
                chars[i] = c;
                let new_hand: String = chars.iter().collect();
                let score = hand_score_joker(new_hand.as_str(), Some(card_score));
                if score > max_score {
                    max_score = score;
                }
            }
            chars[i] = 'J';
        }
    }

    max_score
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut parsed = input
        .trim()
        .split("\n")
        .map(|l| {
            let mut parts = l.split_whitespace();
            let hand = parts.next().unwrap();
            let bid = parts.next().unwrap().parse::<u32>().unwrap();

            (hand.to_string(), bid, hand_score_joker(&hand, None))
        })
        .collect::<Vec<_>>();

    parsed.sort_by_key(|&(_, _, s)| s);

    let sum1: u32 = parsed
        .iter()
        .enumerate()
        .map(|(i, (_, b, _))| b * (i as u32 + 1))
        .sum();

    println!("Sum 1: {}", sum1);
}
