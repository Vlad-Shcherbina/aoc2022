fn parse_range(s: &str) -> (i32, i32) {
    let (a, b) = s.split_once('-').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut overlaps = 0;
    for line in input.split_terminator('\n') {
        let (first, second) = line.split_once(',').unwrap();
        let first = parse_range(first);
        let second = parse_range(second);
        if first.0 <= second.0 && first.1 >= second.1 ||
           first.0 >= second.0 && first.1 <= second.1 {
            overlaps += 1;
        }
    }
    out(overlaps.to_string());
}
