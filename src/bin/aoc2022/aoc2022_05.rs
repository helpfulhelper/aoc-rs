use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{anychar, digit1, line_ending, multispace1, space1};
use nom::combinator::{eof, map, map_parser, map_res, opt};
use nom::multi::{many1, many_till, separated_list0};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom::IResult;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
pub struct AoC2022_05 {
    stacks: Vec<Vec<char>>,
    orders: Vec<(u32, u32, u32)>,
}

impl AoC2022_05 {
    pub fn new() -> Self {
        let lines = read_to_string("../input/2022/05.txt").expect("unable to open file");
        AoC2022_05::parser(&lines)
    }

    pub fn stack_tops(&self) -> String {
        self.stacks
            .iter()
            .flat_map(|s| s.last())
            .collect::<String>()
    }

    pub fn exec_move(&mut self, qty: usize, from: usize, to: usize) {
        for _ in 0..qty {
            let c = self.stacks[from - 1].pop().unwrap();
            self.stacks[to - 1].push(c);
        }
    }
    pub fn exec_move2(&mut self, qty: usize, from: usize, to: usize) {
        let slice_pivot = self.stacks[from - 1].len() - qty;
        let slice = self.stacks[from - 1].split_off(slice_pivot);
        self.stacks[to - 1].extend(slice);
    }

    pub fn parser(lines: &str) -> Self {
        let (_, (cr, _, mv)) = tuple((parse_crates, multispace1, parse_moves))(lines).unwrap();

        Self {
            stacks: cr,
            orders: mv,
        }
    }
}

fn parse_crates(lines: &str) -> IResult<&str, Vec<Vec<char>>> {
    map(many_till(p_crate_line, p_num_line), |(stacks, _)| {
        let num_stacks = stacks.len();
        let mut parsed_crates = vec![Vec::with_capacity(num_stacks); stacks[num_stacks - 1].len()];

        for row in stacks.into_iter().rev() {
            for (index, opt_crate) in row.into_iter().enumerate() {
                if let Some(c) = opt_crate {
                    parsed_crates[index].push(c);
                }
            }
        }
        parsed_crates
    })(lines)
}

fn parse_moves(lines: &str) -> IResult<&str, Vec<(u32, u32, u32)>> {
    map(many_till(p_move, alt((line_ending, eof))), |(m, _)| m)(lines)
}

fn p_move(line: &str) -> IResult<&str, (u32, u32, u32)> {
    map(
        terminated(
            tuple((
                preceded(tuple((tag("move"), space1)), p_num),
                preceded(tuple((space1, tag("from"), space1)), p_num),
                preceded(tuple((space1, tag("to"), space1)), p_num),
            )),
            alt((line_ending, eof)),
        ),
        |x| x,
    )(line)
}
fn p_num_line(line: &str) -> IResult<&str, ()> {
    map(
        terminated(
            many1(delimited(opt(space1), digit1, space1)),
            alt((line_ending, eof)),
        ),
        |_| (),
    )(line)
}

fn p_crate_line(line: &str) -> IResult<&str, Vec<Option<char>>> {
    terminated(separated_list0(tag(" "), p_crate), alt((line_ending, eof)))(line)
}

fn p_crate(s: &str) -> IResult<&str, Option<char>> {
    map_parser(take(3usize), opt(delimited(tag("["), anychar, tag("]"))))(s)
}

fn p_num(s: &str) -> IResult<&str, u32> {
    map_res(digit1, u32::from_str)(s)
}
impl crate::Runner for AoC2022_05 {
    fn parse(&mut self) {
        // let lines = read_to_string("../input/2022/05.txt").expect("unable to open file");
    }

    //year, day
    fn name(&self) -> (usize, usize) {
        (2022, 05)
    }

    fn part1(&mut self) -> Vec<String> {
        for m in self.orders.clone() {
            let (a, b, c) = m;
            self.exec_move(a as usize, b as usize, c as usize);
        }
        crate::output(self.stack_tops())
    }

    fn part2(&mut self) -> Vec<String> {
        for m in self.orders.clone() {
            let (a, b, c) = m;
            self.exec_move2(a as usize, b as usize, c as usize);
        }
        crate::output(self.stack_tops())
    }
}
