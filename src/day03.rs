fn pack(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a',
        b'A'..=b'Z' => c - b'A' + 26,
        _ => panic!(),
    }
}

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut sum = 0;
    let mut seen = [false; 52];
    for line in input.split_terminator('\n') {
        let (left, right) = line.as_bytes().split_at(line.len() / 2);
        seen.fill(false);
        for &c in left {
            seen[pack(c) as usize] = true;
        }
        for &c in right {
            if seen[pack(c) as usize] {
                sum += pack(c) as i32 + 1;
                break;
            }
        }
    }
    out(sum.to_string());
}
