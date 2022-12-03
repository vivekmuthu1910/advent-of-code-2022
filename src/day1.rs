pub fn problem1(input: &str) -> String {
    input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}

pub fn problem2(input: &str) -> String {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|cal| cal.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();
    calories.sort();
    calories.iter().rev().take(3).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_input() -> &'static str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }
    #[test]
    fn check_problem1() {
        let output = "24000";
        assert_eq!(output, &problem1(gen_input()));
    }

    #[test]
    fn check_problem2() {
        let output = "45000";
        assert_eq!(output, &problem2(gen_input()));
    }
}
