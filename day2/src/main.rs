mod input;
use input::Hand;

fn main() {
    let mut games = input::get_part1();
    let mut total = compute_score(&games);
    println!("Part 1: {:#?}", total);

    games = vec![];

    let guide = input::get_part2();
    // Convert the guide into what we need to throw, and store it back into games.
    for game in guide {
        match game {
            // If it's a draw, they played the same move as the opponent.
            (_, Outcome::Draw) => games.push((game.0, game.0)),

            (Hand::Rock, Outcome::Win) => games.push((game.0, Hand::Paper)),
            (Hand::Paper, Outcome::Win) => games.push((game.0, Hand::Scissors)),
            (Hand::Scissors, Outcome::Win) => games.push((game.0, Hand::Rock)),

            (Hand::Rock, Outcome::Loss) => games.push((game.0, Hand::Scissors)),
            (Hand::Paper, Outcome::Loss) => games.push((game.0, Hand::Rock)),
            (Hand::Scissors, Outcome::Loss) => games.push((game.0, Hand::Paper)),
        }
    }
    total = compute_score(&games);
    println!("Part 2: {:#?}", total);
}

fn compute_score(games: &Vec<(Hand, Hand)>) -> u32 {
    let mut total: u32 = 0;
    for game in games {
        total += outcome_to_score(outcome(&game));
        total += hand_to_score(&game.1);
    }
    total
}

pub enum Outcome {
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

/// We are the player on the right.
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
