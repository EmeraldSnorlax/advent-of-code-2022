mod input;

fn main() {
    let mut full_contains: u32 = 0;
    let mut any_overlap: u32 = 0;
    for pair in input::get() {
        let wider = wider_range(&pair[0], &pair[1]);
        let narrower = if wider == &pair[0] { &pair[1] } else { &pair[0] };
        // println!("Wider: {:#?}", wider);
        // println!("Narrower: {:#?}", narrower);
        if fully_contains(wider, narrower) { full_contains += 1 }

        // Any overlap?
        if narrower.to_owned().any(|z| wider.contains(&z)) { any_overlap += 1 };

    }
    println!("Pairs where one fully contains the other: {:#?}", full_contains);
    println!("Pairs where there is any overlap: {:#?}", any_overlap);
}


/// Returns the wider range. If they're equal, it doesn't matter and we return the right.
fn wider_range<'a>(left_range: &'a std::ops::RangeInclusive<i32>, right_range: &'a std::ops::RangeInclusive<i32>) -> &'a std::ops::RangeInclusive<i32> {
    let left_length = left_range.clone().collect::<Vec<i32>>().len();
    let right_length = right_range.clone().collect::<Vec<i32>>().len();
    if  left_length > right_length { return left_range }
    else { return right_range }
}

/// Checks if the left can fully hold the right.
fn fully_contains(base: &std::ops::RangeInclusive<i32>, check_if_inside: &std::ops::RangeInclusive<i32>) -> bool {
    for value in check_if_inside.clone() {
        if !base.contains(&value) { return false }
    }
    true
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn range_inside() {
        assert!(fully_contains(&(28..=32), &(28..=30)));
        assert!(fully_contains(&(0..=1), &(0..=1)));
        assert!(fully_contains(&(0..=1), &(1..=1)));
        assert!(fully_contains(&(0..=48), &(24..=42)));
        assert!(fully_contains(&(4..=6), &(6..=6)));
    }
    #[test]
    fn range_outside() {
        assert!(!fully_contains(&(0..=1), &(1..=2)));
        assert!(!fully_contains(&(0..=1), &(69..=420)));
        assert!(!fully_contains(&(30..=40), &(25..=45)));
        assert!(!fully_contains(&(5..=48), &(6..=49)));
        assert!(!fully_contains(&(100..=200), &(99..=199)));
    }
    #[test]
    fn get_wider_range() {
        assert_eq!(wider_range(&(0..=1), &(0..=50)), &(0..=50));
        assert_eq!(wider_range(&(20..=50), &(25..=50)), &(20..=50));
    }
}
