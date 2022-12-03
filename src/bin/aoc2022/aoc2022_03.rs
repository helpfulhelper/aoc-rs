// use aoc::read_to_chars;
use aoc::read_lines;
use std::collections::HashSet;

trait Score {
    fn score(&self) -> usize;
}

impl Score for char {
    fn score(&self) -> usize {
        (match self {
            'a'..='z' => (*self as u8) - b'a' + 1,
            'A'..='Z' => (*self as u8) - b'A' + 27,
            _ => panic!("BAD"),
        }) as usize
    }
}

pub struct AoC2022_03 {
    data: Vec<String>,
}

impl AoC2022_03 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2022_03 {
    fn parse(&mut self) {
        self.data = read_lines("../input/2022/03.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 03)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        for l in &self.data {
            let (left, right) = l.split_at(l.len() / 2);
            let lset: HashSet<char> = HashSet::from_iter(left.chars());
            let rset: HashSet<char> = HashSet::from_iter(right.chars());
            total += (&lset & &rset).iter().next().unwrap().score();
        }
        crate::output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for triple in self.data.chunks(3) {
            let vec_set: Vec<HashSet<char>> = triple
                .iter()
                .map(|temp| HashSet::from_iter(temp.chars()))
                .collect();

            total += (&(&vec_set[0] & &vec_set[1]) & &vec_set[2])
                .iter()
                .next()
                .unwrap()
                .score();
        }
        crate::output(total)
    }
}
