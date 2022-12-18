pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut it = input.split_terminator('\n');
    let mut monkeys: Vec<Monkey> = vec![];
    loop {
        let n: usize = it.next().unwrap()
            .strip_prefix("Monkey ").unwrap()
            .strip_suffix(':').unwrap()
            .parse().unwrap();
        assert_eq!(n, monkeys.len());
        let line = it.next().unwrap();
        let items: Vec<i32> = line.strip_prefix("  Starting items: ").unwrap()
            .split(", ").map(|s| s.parse().unwrap())
            .collect();
        let line = it.next().unwrap();
        let op = Op::parse(line.strip_prefix("  Operation: new = ").unwrap());
        let div = it.next().unwrap().strip_prefix("  Test: divisible by ").unwrap().parse().unwrap();
        let if_true = it.next().unwrap().strip_prefix("    If true: throw to monkey ").unwrap().parse().unwrap();
        let if_false = it.next().unwrap().strip_prefix("    If false: throw to monkey ").unwrap().parse().unwrap();
        assert_ne!(if_true, n);
        assert_ne!(if_false, n);
        monkeys.push(Monkey {
            items,
            op,
            div,
            if_true,
            if_false,
        });
        match it.next() {
            None => break,
            Some(s) => assert_eq!(s, ""),
        }
    }
    let mut inspect_cnts: Vec<usize> = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let Monkey {
                ref mut items,
                op,
                div,
                if_true,
                if_false,
            } = monkeys[i];
            let items = std::mem::take(items);
            inspect_cnts[i] += items.len();
            for mut item in items {
                item = op.apply(item);
                item /= 3;
                if item % div == 0 {
                    monkeys[if_true].items.push(item);
                } else {
                    monkeys[if_false].items.push(item);
                }
            }
        }
    }
    inspect_cnts.sort_unstable();
    inspect_cnts.reverse();
    let monkey_business = inspect_cnts[0] * inspect_cnts[1];
    out(monkey_business.to_string());
}

struct Monkey {
    items: Vec<i32>,
    op: Op,
    div: i32,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Copy)]
enum Op {
    Add(i32),
    Mul(i32),
    Sqr,
}

impl Op {
    fn parse(s: &str) -> Self {
        if s == "old * old" {
            Op::Sqr
        } else if let Some(s) = s.strip_prefix("old + ") {
            Op::Add(s.parse().unwrap())
        } else if let Some(s) = s.strip_prefix("old * ") {
            Op::Mul(s.parse().unwrap())
        } else {
            panic!("{s:?}")
        }
    }

    fn apply(&self, old: i32) -> i32 {
        match self {
            Op::Add(n) => old + n,
            Op::Mul(n) => old * n,
            Op::Sqr => old * old,
        }
    }
}