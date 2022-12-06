use aoc::read_to_chars;
use itertools::Itertools;
// use aoc::read_lines;

pub struct AoC2022_06 {
    data: Vec<char>,
}

impl AoC2022_06 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2022_06 {
    fn parse(&mut self) {
        self.data = read_to_chars("../input/2022/06.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 06)
    }

    fn part1(&mut self) -> Vec<String> {
        let uniq = self
            .data
            .clone()
            .windows(4)
            .flat_map(<&[char; 4]>::try_from)
            .find_map(|&a| {
                if a.iter().unique().collect::<Vec<&char>>().len() == 4 {
                    Some(a)
                } else {
                    None
                }
            })
            .unwrap()
            .into_iter()
            .collect::<String>();
        println!("{:?}", uniq);
        let packets = self
            .data
            .clone()
            .into_iter()
            .collect::<String>()
            .find(&uniq);
        println!("{:?}", packets.unwrap());
        crate::output(packets.unwrap() + 4)
    }

    fn part2(&mut self) -> Vec<String> {
        let uniq = self
            .data
            .clone()
            .windows(14)
            .flat_map(<&[char; 14]>::try_from)
            .find_map(|&a| {
                if a.iter().unique().collect::<Vec<&char>>().len() == 14 {
                    Some(a)
                } else {
                    None
                }
            })
            .unwrap()
            .into_iter()
            .collect::<String>();
        println!("{:?}", uniq);
        let packets = self
            .data
            .clone()
            .into_iter()
            .collect::<String>()
            .find(&uniq);
        println!("{:?}", packets.unwrap());
        crate::output(packets.unwrap() + 14)
    }
}
