fn pack(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a',
        b'A'..=b'Z' => c - b'A' + 26,
        _ => panic!(),
    }
}

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let lines: Vec<&[u8]> = input.split_terminator('\n').map(|s| s.as_bytes()).collect();
    let mut part1 = 0;
    let mut seen = [false; 52];
    for &line in &lines {
        let (left, right) = line.split_at(line.len() / 2);
        seen.fill(false);
        for &c in left {
            seen[pack(c) as usize] = true;
        }
        for &c in right {
            if seen[pack(c) as usize] {
                part1 += pack(c) as i32 + 1;
                break;
            }
        }
    }
    out(part1.to_string());
    let mut part2 = 0;
    for group in lines.array_chunks::<3>() {
        let mut mask = [0u8; 52];
        for (i, &line) in group.iter().enumerate() {
            for &c in line {
                mask[pack(c) as usize] |= 1 << i;
            }
        }
        part2 += mask.iter().position(|&m| m == 0b111).unwrap() + 1;
    }
    out(part2.to_string());
}
