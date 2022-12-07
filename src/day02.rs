pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.split_terminator('\n') {
        let mut it = line.chars();
        let first = it.next().unwrap();
        assert_eq!(it.next().unwrap(), ' ');
        let second = it.next().unwrap();
        assert!(it.next().is_none());

        let first = first as i32 - 'A' as i32;
        let second = second as i32 - 'X' as i32;
        assert!((0..3).contains(&first));
        assert!((0..3).contains(&second));
        let win_score = match (second + 3 - first) % 3 {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => unreachable!()
        };
        part1 += second + 1 + win_score;
        part2 += (first + second + 2) % 3 + 1 + second * 3;
    }
    out(part1.to_string());
    out(part2.to_string());
}
