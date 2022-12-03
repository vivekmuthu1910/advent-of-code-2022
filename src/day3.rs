use itertools::Itertools;

fn value_of_char(ch: u8) -> u32 {
    if ch >= 'A' as u8 && ch <= 'Z' as u8 {
        (ch - 0x40 + 26) as u32
    } else if ch >= 'a' as u8 && ch <= 'z' as u8 {
        (ch - 0x60) as u32
    } else {
        0
    }
}

fn split_and_count(line: &str) -> u32 {
    let len = line.len();
    if len % 2 != 0 {
        panic!("Not a even number of items");
    }
    let slot1 = &line.as_bytes()[..len / 2];
    let slot2 = &line.as_bytes()[len / 2..];

    for ch in slot1 {
        if slot2.contains(ch) {
            return value_of_char(*ch);
        }
    }
    0
}

pub fn problem1(input: &str) -> String {
    input.lines().map(split_and_count).sum::<u32>().to_string()
}

pub fn problem2(input: &str) -> String {
    input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|elves| {
            for ch in elves.0.as_bytes() {
                if elves.1.as_bytes().contains(ch) && elves.2.as_bytes().contains(ch) {
                    return value_of_char(*ch);
                }
            }
            0
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
        let output = "157";
        assert_eq!(output, &problem1(gen_input()));
    }

    #[test]
    fn check_problem2() {
        let output = "70";
        assert_eq!(output, &problem2(gen_input()));
    }
}
