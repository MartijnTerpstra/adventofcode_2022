use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    calculate_monkey_business(Some(3), 20);
    calculate_monkey_business(None, 10000);
}

fn calculate_monkey_business(worry_level: Option<i64>, rounds: i64) {
    let mut monkeys = load_monkeys();

    for _ in 0..rounds {
        handle_monkey_business(&mut monkeys, worry_level);
    }
    let monkey_business: usize = monkeys
        .into_iter()
        .sorted_by(|l, r| l.inspects.cmp(&r.inspects))
        .rev()
        .take(2)
        .map(|e| e.inspects)
        .product();

    println!(
        "Monkey business after {} rounds: {}",
        rounds, monkey_business
    );
}

fn handle_monkey_business(monkeys: &mut [Monkey], worry_level: Option<i64>) {
    let division_by_product: i64 = monkeys.iter().map(|e| e.division_by).product();

    for monkey_index in 0..monkeys.len() {
        monkeys[monkey_index].inspects += monkeys[monkey_index].items.len();
        while !monkeys[monkey_index].items.is_empty() {
            let item = monkeys[monkey_index].items.pop_front().unwrap();
            let mut worry_result = monkeys[monkey_index].operation.do_operation(item);
            if worry_level.is_some() {
                worry_result /= worry_level.unwrap();
            } else {
                worry_result %= division_by_product;
            }
            let test_result = worry_result % monkeys[monkey_index].division_by == 0;
            if test_result {
                let next_monkey = &mut monkeys[monkeys[monkey_index].if_true_throw_to_monkey];
                next_monkey.items.push_back(worry_result);
            } else {
                let next_monkey = &mut monkeys[monkeys[monkey_index].if_false_throw_to_monkey];
                next_monkey.items.push_back(worry_result);
            }
        }
    }
}

fn load_monkeys() -> Vec<Monkey> {
    let monkey_re = Regex::new(
        r"Monkey \d+:
  Starting items: (.*)
  Operation: new = (.*)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)",
    )
    .unwrap();

    let monkey_input = read_text("input.txt").unwrap();

    let mut monkeys = Vec::new();

    for monkey_match in monkey_re.captures_iter(monkey_input.as_str()) {
        let starting_items = monkey_match[1]
            .split(',')
            .map(|e| e.trim().to_string().parse().unwrap())
            .collect();

        let operation = Operation::new(&monkey_match[2]);

        monkeys.push(Monkey {
            items: starting_items,
            operation: operation,
            division_by: monkey_match[3].parse().unwrap(),
            if_true_throw_to_monkey: monkey_match[4].parse().unwrap(),
            if_false_throw_to_monkey: monkey_match[5].parse().unwrap(),
            inspects: 0,
        });
    }

    println!("Done");

    return monkeys;
}

struct Operation {
    operator: char,
    left: Option<i64>,
    right: Option<i64>,
}

impl Operation {
    fn new(operation_txt: &str) -> Operation {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(old|\d+) ([+*]) (old|\d+)").unwrap();
        }
        let captures = RE.captures(operation_txt).unwrap();

        let op = captures[2].chars().next().unwrap();
        let l = match &captures[1] {
            "old" => None,
            number => Some(number.parse().unwrap()),
        };
        let r = match &captures[3] {
            "old" => None,
            number => Some(number.parse().unwrap()),
        };
        return Operation {
            operator: op,
            left: l,
            right: r,
        };
    }

    fn do_operation(&self, item: i64) -> i64 {
        match self.operator {
            '*' => self.left.unwrap_or(item) * self.right.unwrap_or(item),
            '+' => self.left.unwrap_or(item) + self.right.unwrap_or(item),
            _ => panic!("Unknown operator: {}", self.operator),
        }
    }
}

struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    division_by: i64,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
    inspects: usize,
}

fn read_text<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut s = String::new();
    return io::BufReader::new(file)
        .read_to_string(&mut s)
        .and_then(|_| Ok(s));
}
