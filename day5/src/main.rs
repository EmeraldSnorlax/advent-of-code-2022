mod input;
fn main() {
    let (mut yard, operations) = input::get();
    for operation in &operations {
        for _ in 1..=operation.count {
            let moved_item = yard[operation.from as usize].pop().unwrap();
            yard[operation.to as usize].push(moved_item);
        }
    }
    print!{"Part 1: "}
    for stack in &yard {
        print!("{:#?}", stack.last().unwrap());
    }

    // Reset the yard for part 2
     (yard, ..) = input::get();
     for operation in &operations {
        let mut grabbed_items = vec![];
        for _ in 1..=operation.count {
            grabbed_items.push(yard[operation.from as usize].pop().unwrap());
        }
        // We need to reverse it, because we are still pushing to the end one at a time.
        yard[operation.to as usize].extend(grabbed_items.into_iter().rev());
    }
    print!{"\nPart 2: "}
    for stack in &yard {
        print!("{:#?}", stack.last().unwrap());
    }
}