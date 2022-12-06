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
        crate::output(solve(4, self.data.clone()))
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(solve(14, self.data.clone()))
    }
}

fn solve(win: usize, data: Vec<char>) -> usize {
    let uniq = find_first_unique(win, data.clone());
    find_num_processed(win, data, &uniq)
}

fn find_first_unique(win: usize, data: Vec<char>) -> String {
    let uniq = data
        .clone()
        .windows(win)
        .flat_map(<Vec<char>>::try_from)
        .find_map(|a| {
            if a.iter().unique().collect::<Vec<&char>>().len() == win {
                Some(a)
            } else {
                None
            }
        })
        .unwrap()
        .into_iter()
        .collect::<String>();
    println!("{:?}", uniq);
    uniq
}
fn find_num_processed(win: usize, data: Vec<char>, uniq: &str) -> usize {
    let packets = data.clone().into_iter().collect::<String>().find(&uniq);
    packets.unwrap() + win
}
