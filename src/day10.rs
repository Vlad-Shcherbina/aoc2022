pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let ts = [20, 60, 100, 140, 180, 220];
    let mut t = 1;
    let mut x: i32 = 1;
    let mut res = 0;
    let mut screen = String::new();
    for line in input.split_terminator('\n') {
        let dt;
        let dx: i32;
        if line == "noop" {
            dt = 1;
            dx = 0;
        } else if let Some(arg) = line.strip_prefix("addx ") {
            dt = 2;
            dx = arg.parse().unwrap();
        } else {
            panic!();
        }
        for _ in 0..dt {
            if t % 40 == 1 {
                screen.push('\n');
            }
            if ts.contains(&t) {
                res += t * x;
            }
            if ((t - 1) % 40 - x).abs() <= 1 {
                screen.push('#');
            } else {
                screen.push('.');
            }
            t += 1;
        }
        x += dx;
    }
    out(res.to_string());
    out(screen);
}
