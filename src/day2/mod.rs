#[derive(Debug, Clone, PartialEq)]
enum Play {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}
impl From<&str> for Play {
    fn from(f: &str) -> Self {
        match f {
            "A" => Play::Rock,
            "X" => Play::Rock,
            "B" => Play::Paper,
            "Y" => Play::Paper,
            "C" => Play::Scissors,
            "Z" => Play::Scissors,
            _ => panic!("Unknown Play '{}'", f),
        }
    }
}
impl Play {
    fn value(&self) -> u8 {
        match *self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }

    fn beats(&self, other: &Play) -> bool {
        match *self {
            Play::Rock => match other {
                Play::Scissors => true,
                _ => false,
            },
            Play::Paper => match other {
                Play::Rock => true,
                _ => false,
            },
            Play::Scissors => match other {
                Play::Paper => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}
impl From<&str> for Outcome {
    fn from(f: &str) -> Self {
        match f {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown Outcome '{}'", f),
        }
    }
}
impl Outcome {
    fn value(&self) -> u8 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Round {
    opponent: Play,
    me: Play,
}

impl Round {
    fn score(&self) -> u8 {
        self.me.value() + self.outcome().value()
    }

    fn outcome(&self) -> Outcome {
        if self.me == self.opponent {
            return Outcome::Draw;
        }
        if self.me.beats(&self.opponent) {
            return Outcome::Win;
        }
        Outcome::Lose
    }
}

fn parse_file(text: &str) -> Vec<Round> {
    let round_lines = text.split("\n");
    let rounds = round_lines
        .map(|line| {
            let plays: Vec<&str> = line.split(" ").collect();
            Round {
                opponent: plays.first().unwrap().to_owned().into(),
                me: plays.last().unwrap().to_owned().into(),
            }
        })
        .collect();

    rounds
}

fn parse_file_2(text: &str) -> Vec<Round> {
    let round_lines = text.split("\n");
    let rounds = round_lines
        .map(|line| {
            let plays: Vec<&str> = line.split(" ").collect();
            let opponent: Play = plays.first().unwrap().to_owned().into();
            let outcome: Outcome = plays.last().unwrap().to_owned().into();
            let me: Play = match outcome {
                Outcome::Win => match opponent {
                    Play::Rock => Play::Paper,
                    Play::Paper => Play::Scissors,
                    Play::Scissors => Play::Rock,
                },
                Outcome::Lose => match opponent {
                    Play::Rock => Play::Scissors,
                    Play::Paper => Play::Rock,
                    Play::Scissors => Play::Paper,
                },
                Outcome::Draw => opponent.clone(),
            };
            Round { opponent, me }
        })
        .collect();

    rounds
}

fn score_rounds(rounds: Vec<Round>) -> u32 {
    rounds.iter().fold(0, |acc, r| acc + u32::from(r.score()))
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_STR: &str = "A Y
B X
C Z";

    #[test]
    fn test_parse_file() {
        let rounds = parse_file(TEST_STR);
        assert_eq!(rounds.len(), 3);
        assert_eq!(rounds.first().unwrap().opponent, Play::Rock);
        assert_eq!(rounds.first().unwrap().me, Play::Paper);
    }

    #[test]
    fn test_score_rounds() {
        assert_eq!(score_rounds(parse_file(TEST_STR)), 15);
    }
}

#[allow(dead_code)]
pub fn part1() {
    let file_path = format!(
        "src/{}/input.txt",
        module_path!().split("::").last().unwrap()
    )
    .to_owned();
    let path = Path::new(file_path.as_str());
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("{}", score_rounds(parse_file(contents.as_str())))
}

#[allow(dead_code)]
pub fn part2() {
    let contents = super::read_file(module_path!());

    println!("{}", score_rounds(parse_file_2(contents.as_str())))
}
