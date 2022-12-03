use std::collections::HashSet;

mod input;

/// A-Z and a-z.
type Item = char;

#[derive(Debug, Clone)]
struct Rucksack {
    front: HashSet<Item>,
    back: HashSet<Item>,
    common: Item,
}

#[derive(Debug)]
struct ElfGroup {
    elves: (HashSet<Item>, HashSet<Item>, HashSet<Item>),
    badge: Item,
}

fn main() {
    let mut rucksacks: Vec<Rucksack> = vec![];
    for line in input::get() {
        rucksacks.push(construct_rucksack(line));
    }
    let mut priority_total: u32 = 0;
    for rucksack in &rucksacks {
        priority_total += priority(&rucksack.common) as u32;
    }
    println!("Sum of priorities for part 1: {}", priority_total);

    let mut groups: Vec<ElfGroup> = vec![];
    for rucksack in rucksacks.chunks(3) {
        let mut group = ElfGroup {
            elves: (
                // Make a HashSet for each rucksack, containing every single item in both pockets.
                HashSet::from_iter(
                    rucksack[0]
                        .front
                        .to_owned()
                        .into_iter()
                        .chain(rucksack[0].back.to_owned()),
                ),
                HashSet::from_iter(
                    rucksack[1]
                        .front
                        .to_owned()
                        .into_iter()
                        .chain(rucksack[1].back.to_owned()),
                ),
                HashSet::from_iter(
                    rucksack[2]
                        .front
                        .to_owned()
                        .into_iter()
                        .chain(rucksack[2].back.to_owned()),
                ),
            ),
            badge: ' ',
        };
        // Finally, intersect all the bags to find the badge.
        group.badge = group
            .elves
            .0
            .intersection(&group.elves.1)
            .cloned()
            .collect::<HashSet<Item>>()
            .intersection(&group.elves.2)
            .collect::<Vec<&Item>>()[0]
            .to_owned();
        groups.push(group);
    }
    // Finally, get the sum of priorities of the badge.
    priority_total = 0;
    for group in groups {
        priority_total += priority(&group.badge) as u32
    }
    println!("Sum of badge priority: {:#?}", priority_total);
}

fn construct_rucksack(line: &str) -> Rucksack {
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
    rucksack.common = rucksack
        .front
        .intersection(&rucksack.back)
        .nth(0)
        .unwrap()
        .to_owned();
    rucksack
}

fn priority(item: &Item) -> u8 {
    match item {
        'a'..='z' => (*item as u8) - 96,
        // Otherwise it is uppercase.
        'A'..='Z' => (*item as u8) - 64 + 26,
        // Can't get others.
        _ => panic!("Tried to get priority of non A-Z a-z: {}", item),
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
