// use aoc::read_to_chars;
use aoc::numbers;
// use aoc::read_lines;

pub struct AoC2015_XX {
    data: Vec<T>,
}

impl AoC2015_XX {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2015_XX {
    fn parse(&mut self) {
        let mut data = numbers("../input/2015/XX.txt", 'x');
        // for d in &mut data {
        //     d.sort();
        //     self.data.push(Present::new(d[0], d[1], d[2]))
        // }
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2015, XX)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("0".to_string())
    }
}
