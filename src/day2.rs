fn score1(combo: &str) -> u32 {
    let mut split = combo.split(" ");
    let other = split.next().unwrap();
    let me = split.next().unwrap();

    let mut score = match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Wrong choice"),
    };
    score += match (other, me) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => panic!("Wrong choice"),
    };
    score
}

pub fn problem1(input: &str) -> String {
    input.lines().map(score1).sum::<u32>().to_string()
}

fn score2(combo: &str) -> u32 {
    let mut split = combo.split(" ");
    let other = split.next().unwrap();
    let me = split.next().unwrap();

    let mut score = match me {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Wrong choice"),
    };
    score += match (other, me) {
        ("A", "X") => 3,
        ("A", "Y") => 1,
        ("A", "Z") => 2,
        ("B", "X") => 1,
        ("B", "Y") => 2,
        ("B", "Z") => 3,
        ("C", "X") => 2,
        ("C", "Y") => 3,
        ("C", "Z") => 1,
        _ => panic!("Wrong choice"),
    };
    score
}

pub fn problem2(input: &str) -> String {
    input.lines().map(score2).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_input() -> &'static str {
        "A Y
B X
C Z"
    }

    #[test]
    fn check_problem1() {
        let output = "15";
        assert_eq!(output, &problem1(gen_input()));
    }

    #[test]
    fn check_problem2() {
        let output = "12";
        assert_eq!(output, &problem2(gen_input()));
    }
}
