pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut hx = 0i32;
    let mut hy = 0i32;
    let mut tx = 0;
    let mut ty = 0;
    let mut visited = vec![(tx, ty)];
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
            hx += dx;
            hy += dy;
            if (hx - tx).abs() > 1 || (hy - ty).abs() > 1 {
                tx += (hx - tx).signum();
                ty += (hy - ty).signum();
            }
            visited.push((tx, ty));
        }
    }
    visited.sort_unstable();
    visited.dedup();
    out(visited.len().to_string());
}
