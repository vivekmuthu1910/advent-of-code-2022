use std::str::FromStr;

struct MyRange<T> {
    start: T,
    end: T,
}
impl<T: FromStr> FromStr for MyRange<T> {
    type Err = T::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();
        let start = start.parse::<T>()?;
        let end = end.parse::<T>()?;

        Ok(Self { start, end })
    }
}

impl<T: PartialOrd> MyRange<T> {
    fn contains_range(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn do_overlap(&self, other: &Self) -> bool {
        !(self.start > other.end || self.end < other.start)
    }
}

pub fn problem1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let elf1_range: MyRange<usize> = MyRange::from_str(elf1).unwrap();
            let elf2_range = MyRange::from_str(elf2).unwrap();
            (elf1_range, elf2_range)
        })
        .filter(|(elf1, elf2)| elf1.contains_range(&elf2) || elf2.contains_range(&elf1))
        .count()
        .to_string()
}

pub fn problem2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (elf1, elf2) = line.split_once(',').unwrap();
            let elf1_range: MyRange<usize> = MyRange::from_str(elf1).unwrap();
            let elf2_range = MyRange::from_str(elf2).unwrap();
            (elf1_range, elf2_range)
        })
        .filter(|(elf1, elf2)| elf1.do_overlap(&elf2))
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn gen_input() -> &'static str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    }

    #[test]
    fn check_range_contains() {
        let r1 = MyRange { start: 0, end: 43 };
        let r2 = MyRange { start: 5, end: 27 };
        assert!(r1.contains_range(&r2));
    }

    #[test]
    fn check_problem1() {
        assert_eq!("2", &problem1(gen_input()));
    }

    #[test]
    fn check_problem2() {
        assert_eq!("4", &problem2(gen_input()));
    }
}
