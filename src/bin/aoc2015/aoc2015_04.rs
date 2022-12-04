use aoc::read_lines;
use md5;
// use aoc::read_lines;

pub struct AoC2015_04 {
    data: Vec<String>,
}

impl AoC2015_04 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2015_04 {
    fn parse(&mut self) {
        self.data = read_lines("../input/2015/04.txt");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2015, 04)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut sol = 0;
        for x in 1.. {
            let temp = self.data[0].clone() + &x.to_string();
            let digest = format!("{:x}", md5::compute(&temp));
            // if let t = digest.chars().take(5).collect() {

            // }
            break;
        }
        crate::output(sol.to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }
}
