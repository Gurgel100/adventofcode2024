use aoc::{IterUnwrap, Parse};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    calculate_calibration_result(input, &[Operator::Add, Operator::Mul])
}

fn part_2(input: aoc::Input) -> impl ToString {
    calculate_calibration_result(input, &[Operator::Add, Operator::Mul, Operator::Concat])
}

enum Operator {
    Add,
    Mul,
    Concat,
}
impl Operator {
    fn do_op(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
            Operator::Concat => a * 10u64.pow(f64::log10(b as f64) as u32 + 1) + b,
        }
    }
}

fn calculate_calibration_result(input: aoc::Input, operators: &[Operator]) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let mut split = line.split(':');
            let result = split.next_uw().parse_uw::<u64>();
            let values = split
                .next_uw()
                .split_whitespace()
                .map(|v| v.parse_uw())
                .collect::<Vec<u64>>();
            operators
                .iter()
                .any(|op| {
                    check_operator(
                        result,
                        op.do_op(values[0], values[1]),
                        &operators,
                        &values[2..],
                    )
                })
                .then_some(result)
        })
        .sum::<u64>()
}

fn check_operator(expected: u64, current: u64, operators: &[Operator], values: &[u64]) -> bool {
    if values.len() == 0 {
        expected == current
    } else {
        operators.iter().any(|op| {
            let val = op.do_op(current, values[0]);
            expected >= val && check_operator(expected, val, operators, &values[1..])
        })
    }
}
