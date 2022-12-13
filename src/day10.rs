pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let ts = [20, 60, 100, 140, 180, 220];
    let mut t = 1;
    let mut x = 1;
    let mut res = 0;
    for line in input.split_terminator('\n') {
        if line == "noop" {
            if ts.contains(&t) {
                res += t * x;
            }
            t += 1;
        } else if let Some(arg) = line.strip_prefix("addx ") {
            let arg: i32 = arg.parse().unwrap();
            for _ in 0..2 {
                if ts.contains(&t) {
                    res += t * x;
                }
                t += 1;
            }
            x += arg;
        }
    }
    out(res.to_string());
}
