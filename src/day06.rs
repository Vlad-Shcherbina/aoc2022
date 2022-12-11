pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let input = input.trim_end().as_bytes();
    let mut i = 4;
    let result = loop {
        if all_distinct(&input[i - 4 .. i]) {
            break i;
        }
        i += 1;
    };
    out(result.to_string());
}

fn all_distinct(xs: &[u8]) -> bool {
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            if xs[i] == xs[j] {
                return false;
            }
        }
    }
    true
}
