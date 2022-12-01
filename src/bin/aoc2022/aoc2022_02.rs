use aoc::read_to_chars;
// use aoc::read_lines;

pub struct AoC2022_02 {
    data: Vec<char>,
}

impl AoC2022_02 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2022_02 {
    fn parse(&mut self) {
        self.data = read_to_chars("../input/2022/01.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 2)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }
}
