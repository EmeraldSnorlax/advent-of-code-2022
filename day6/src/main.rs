use itertools::Itertools;
use std::collections::{VecDeque};

mod input;

fn main() {
    let input = input::get();
    let mut last: VecDeque<char> = VecDeque::new();
    const PACKET_LENGTH: u16 = 4; // 4 in part 1.
    let mut start: u16 = 0;
    'outer: for (i, c) in input.chars().enumerate() {
        if last.len() < PACKET_LENGTH as usize {
            last.push_back(c);
            continue;
        }
        if last.contains(&c) {
            // drain up to current.
            last.drain(0..);
            last.push_back(c);
            continue;
        };
        let unique_seen = last.iter().unique().count();
        if unique_seen == PACKET_LENGTH as usize {
            start = (i - 2) as u16;
            break 'outer;
        }
        last.pop_front();
        last.push_back(c);

    }
    println!("{:#?}", last);
    println!("Part 1: (Index, number): {:#?}", (start, start + 1));
}
// 1913 too low