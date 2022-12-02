use std::fs;

fn main() {
    let file = fs::read_to_string("./inputs/day_02_input.txt").unwrap();
    println!("{}", score_from_strategy_guide(&file));
    println!("{}", score_from_strategy_guide_part_2(&file));
}

fn score_from_strategy_guide(input: &str) -> u32 {
    // strategy guide part 1 assumes x, y, z is your opponent's move
    let rounds: Vec<&str> = input.lines().collect();

    let mut score = 0;
    for round in rounds {
        let choices: Vec<&str> = round.split(" ").filter(|x| !x.is_empty()).collect();
        // choices[0] = A, B, or C
        // choices[1] = X, Y, or Z
        let opponent = get_variant(choices[0]);
        let my_choice = get_variant(choices[1]);

        let outcome = get_outcome(my_choice, opponent);

        let outcome_points = get_outcome_points(outcome);
        let choice_points = get_point_value(my_choice);

        score += outcome_points + choice_points;
    }

    score
}

fn score_from_strategy_guide_part_2(input: &str) -> u32 {
    // strategy guide part 2 reveals x, y, z is the outcome you need to get

    let rounds: Vec<&str> = input.lines().collect();

    let mut score = 0;
    for round in rounds {
        let choices: Vec<&str> = round.split(" ").filter(|x| !x.is_empty()).collect();
        // choices[0] = A, B, or C
        // choices[1] = X, Y, or Z
        let opponent = get_variant(choices[0]);
        let intended_outcome = get_intended_outcome(choices[1]);
        let my_choice = get_choice(opponent, intended_outcome);

        // let outcome = get_outcome(my_choice, opponent);

        let outcome_points = get_outcome_points(intended_outcome);
        let choice_points = get_point_value(my_choice);

        score += outcome_points + choice_points;
    }

    score
}

#[derive(Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

fn get_variant(letter: &str) -> RPS {
    if letter.contains(['A', 'X']) {
        return RPS::Rock;
    } else if letter.contains(['B', 'Y']) {
        return RPS::Paper;
    } else if letter.contains(['C', 'Z']) {
        return RPS::Scissors;
    } else {
        //error
        dbg!(letter);
        // the error was test input had spaces in front of letters
        // "A, X"
        // "    B, Y"
        // "    C, Z"
        return RPS::Scissors;
    }
}

fn get_variant_part_2(letter: &str) -> RPS {
    if letter.contains('A') {
        return RPS::Rock;
    } else if letter.contains('B') {
        return RPS::Paper;
    } else {
        // if letter.contains('C')
        return RPS::Scissors;
    }
}

fn get_intended_outcome(letter: &str) -> Outcome {
    if letter.contains('X') {
        return Outcome::Lose;
    } else if letter.contains('Y') {
        return Outcome::Draw;
    } else {
        // if letter.contains('C')
        return Outcome::Win;
    }
}

fn get_point_value(selection: RPS) -> u32 {
    match selection {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

fn get_outcome(a: RPS, b: RPS) -> Outcome {
    match a {
        RPS::Rock => match b {
            RPS::Rock => Outcome::Draw,
            RPS::Paper => Outcome::Lose,
            RPS::Scissors => Outcome::Win,
        },
        RPS::Paper => match b {
            RPS::Rock => Outcome::Win,
            RPS::Paper => Outcome::Draw,
            RPS::Scissors => Outcome::Lose,
        },
        RPS::Scissors => match b {
            RPS::Rock => Outcome::Lose,
            RPS::Paper => Outcome::Win,
            RPS::Scissors => Outcome::Draw,
        },
    }
}

fn get_outcome_points(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    }
}

fn get_choice(opp_selection: RPS, outcome: Outcome) -> RPS {
    match outcome {
        Outcome::Win => match opp_selection {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        },
        Outcome::Draw => opp_selection,
        Outcome::Lose => {
            match opp_selection {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn score_of_test_input() {
        let score = score_from_strategy_guide(BASIC_INPUT);

        assert_eq!(15, score);
    }

    #[test]
    fn single_line() {
        let score = score_from_strategy_guide("A Y");

        assert_eq!(8, score);

        let score = score_from_strategy_guide("B X");
        assert_eq!(1, score);

        let score = score_from_strategy_guide("C Z");
        assert_eq!(6, score);
    }

    // part 2
    #[test]
    fn part_2_score() {
        let score = score_from_strategy_guide_part_2(BASIC_INPUT);

        assert_eq!(12, score);
    }

    #[test]
    fn part_2_single_line() {
        let score = score_from_strategy_guide_part_2("A Y");

        assert_eq!(4, score);

        let score = score_from_strategy_guide_part_2("B X");
        assert_eq!(1, score);

        let score = score_from_strategy_guide_part_2("C Z");
        assert_eq!(7, score);
    }
}
