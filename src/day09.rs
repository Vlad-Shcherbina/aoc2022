pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    for rope_len in [2, 10] {
        let mut rope: Vec<(i32, i32)> = vec![(0, 0); rope_len];
        let mut visited = vec![rope[0]];
        for line in input.split_terminator('\n') {
            let (dir, dist) = line.split_once(' ').unwrap();
            let dist: i32 = dist.parse().unwrap();
            let (dx, dy) = match dir {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };
            for _ in 0..dist {
                rope[0].0 += dx;
                rope[0].1 += dy;
                for i in 1..rope.len() {
                    let (hx, hy) = rope[i - 1];
                    let t = &mut rope[i];
                    if (hx - t.0).abs() > 1 || (hy - t.1).abs() > 1 {
                        t.0 += (hx - t.0).signum();
                        t.1 += (hy - t.1).signum();
                    }
                }
                visited.push(*rope.last().unwrap());
            }
        }
        visited.sort_unstable();
        visited.dedup();
        out(visited.len().to_string());
    }
}
