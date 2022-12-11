pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let input = input.trim_end().as_bytes();
    for n in [4, 14] {
        for i in n.. {
            if all_distinct(&input[i - n .. i]) {
                out(i.to_string());
                break;
            }
        }
    }
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
