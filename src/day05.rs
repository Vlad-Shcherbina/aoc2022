pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let (initial, commands) = input.split_once("\n\n").unwrap();
    let mut lines = initial.split_terminator('\n').rev();
    let num_stacks = (lines.next().unwrap().len() + 1) / 4;
    let mut stacks = vec![vec![]; num_stacks];
    for line in lines {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = line.as_bytes()[i * 4 + 1];
            if c != b' ' {
                stack.push(c);
            }
        }
    }
    for command in commands.split_terminator('\n') {
        let command = command.strip_prefix("move ").unwrap();
        let (cnt, rest) = command.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let cnt: usize = cnt.parse().unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = to.parse::<usize>().unwrap() - 1;
        for _ in 0..cnt {
            let t = stacks[from].pop().unwrap();
            stacks[to].push(t);
        }
    }
    let result: String = stacks.iter()
        .map(|s| *s.last().unwrap() as char)
        .collect();
    out(result);
}
