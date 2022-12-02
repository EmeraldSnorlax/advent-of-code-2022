mod input;
use input::Hand;

fn main() {

    let games = input::get();
    let mut total: u32 = 0;

    for game in games {
        total += outcome_to_score(outcome(&game));

        total += hand_to_score(&game.1);
    }
    println!("Part 1: {:#?}", total);
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

fn hand_to_score(hand: &Hand) -> u32 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn outcome_to_score(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

/// We are the right.
fn outcome(game: &(Hand, Hand)) -> Outcome {
    match game {
        (Hand::Rock, Hand::Rock) => Outcome::Draw,
        (Hand::Rock, Hand::Paper) => Outcome::Win,
        (Hand::Rock, Hand::Scissors) => Outcome::Loss,

        (Hand::Paper, Hand::Rock) => Outcome::Loss,
        (Hand::Paper, Hand::Paper) => Outcome::Draw,
        (Hand::Paper, Hand::Scissors) => Outcome::Win,

        (Hand::Scissors, Hand::Rock) => Outcome::Win,
        (Hand::Scissors, Hand::Paper) => Outcome::Loss,
        (Hand::Scissors, Hand::Scissors) => Outcome::Draw,
    }
}
