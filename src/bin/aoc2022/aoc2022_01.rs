use aoc::read_lines2;

pub struct AoC2022_01 {
    data: Vec<Elf>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Elf {
    cals: u32,
}

impl std::fmt::Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cals)
    }
}

impl Elf {
    pub fn new(c: Vec<String>) -> Self {
        let mut t: u32 = 0;
        for b in c {
            t += b.parse::<u32>().unwrap();
        }

        Self { cals: t }
    }
}

impl AoC2022_01 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl crate::Runner for AoC2022_01 {
    fn parse(&mut self) {
        let temp = read_lines2("input/2022-01.txt");
        let mut t: Vec<String> = Vec::new();
        for x in temp {
            if x != "" {
                t.push(x);
            } else {
                self.data.push(Elf::new(t.clone()));
                t.clear();
            }
        }

        self.data.sort();
        // print!("{:?}", self.data);
        // print!("{:?}", x);
    }

    fn name(&self) -> (usize, usize) {
        (2022, 1)
    }

    fn part1(&mut self) -> Vec<String> {
        // crate::output(self.data)
        crate::output(self.data.last().unwrap().cals)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output(
            self.data.pop().unwrap().cals
                + self.data.pop().unwrap().cals
                + self.data.pop().unwrap().cals,
        )
    }
}
