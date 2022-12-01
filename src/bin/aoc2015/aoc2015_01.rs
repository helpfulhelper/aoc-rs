use aoc::read_to_chars;

pub struct AoC2015_01 {
    data: Vec<char>,
}

impl AoC2015_01 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2015_01 {
    fn parse(&mut self) {
        self.data = read_to_chars("input/2015/01.txt");
    }

    fn name(&self) -> (usize, usize) {
        (2015, 1)
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(
            self.data
                .iter()
                .map(|x| match x {
                    '(' => 1,
                    ')' => -1,
                    _ => panic!("invalid char in input"),
                })
                .sum::<i32>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut cur_floor = 0;
        for (pos, c) in self.data.iter().enumerate() {
            let step = match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("invalid character input"),
            };
            cur_floor += step;
            if cur_floor < 0 {
                return crate::output(pos + 1);
            }
        }

        panic!("no answer found");
    }
}
