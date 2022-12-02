mod input;
fn main() {
    let input = input::get();
    let mut sum_of_each_elf: Vec<u64> = vec![];
    for elf in input {
        sum_of_each_elf.push(elf.iter().sum());
    }
    sum_of_each_elf.sort();
    let top: Vec<&u64> = sum_of_each_elf.iter().rev().take(3).collect();
    println!("Top 1: {:#?}", top[0]);
    println!("Top 3 total: {:#?}", top.into_iter().sum::<u64>());
}
