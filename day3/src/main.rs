use std::collections::HashSet;

mod input;

/// A-Z and a-z.
type Item = char;

#[derive(Debug)]
struct Rucksack {
    front: HashSet<Item>,
    back: HashSet<Item>,
    common: Item,
}

fn main() {
    let mut rucksacks: Vec<Rucksack> = vec![];
    for line in input::get() {
        let middle = line.len() / 2;
        let items: Vec<Item> = line.chars().collect();
        let front = &items[..middle];
        let back = &items[middle..];
        let mut rucksack = Rucksack {
            front: HashSet::from_iter(front.to_vec()),
            back: HashSet::from_iter(back.to_vec()),
            common: ' ',
        };
        // We can unwrap because we will definitely have an overlap according to our input.
        rucksack.common = rucksack.front.intersection(&rucksack.back).nth(0).unwrap().to_owned();
        rucksacks.push(rucksack);
    }
    println!("{:#?}", rucksacks);

    let mut priority_total: u32 = 0;
    for rucksack in rucksacks {
        priority_total += priority(&rucksack.common) as u32;
    }
    println!("Sum of priorities: {}", priority_total);
}

fn priority(item: &Item) -> u8 {
    match item {
        'a'..='z' => (*item as u8) - 96,
        // Otherwise it is uppercase.
        'A'..='Z' => (*item as u8) - 64 + 26,
        // Can't get others.
        _ => panic!("Tried to get priority of non A-Z a-z: {}", item)
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_priories() {
        assert_eq!(priority(&'a'), 1);
        assert_eq!(priority(&'z'), 26);
        assert_eq!(priority(&'A'), 27);
        assert_eq!(priority(&'Z'), 52);
    }
    #[test]
    #[should_panic]
    fn invalid_priorities() {
        priority(&'0');
    }
}