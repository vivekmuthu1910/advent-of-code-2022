use itertools::Itertools;

fn value_of_char(ch: char) -> u32 {
    match ch {
        'A'..='Z' => (ch as u8 - 0x26) as u32,
        'a'..='z' => (ch as u8 - 0x60) as u32,
        _ => 0,
    }
}

pub fn problem1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let slot1 = &line[..line.len() / 2];
            let slot2 = &line[line.len() / 2..];

            let common_char = slot1.chars().find(|c| slot2.contains(*c)).unwrap();
            value_of_char(common_char)
        })
        .sum::<u32>()
        .to_string()
}

pub fn problem2(input: &str) -> String {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|(elf0, elf1, elf2)| {
            let common_char = elf0
                .chars()
                .find(|c| elf1.contains(*c) && elf2.contains(*c))
                .unwrap();
            value_of_char(common_char)
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_input() -> &'static str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    #[test]
    fn check_problem1() {
        assert_eq!("157", &problem1(gen_input()));
    }

    #[test]
    fn check_problem2() {
        assert_eq!("70", &problem2(gen_input()));
    }
}
