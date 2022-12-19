pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let w = 600;
    let h = 200;
    let mut board = vec![b'.'; w * h];
    for line in input.lines() {
        let parts: Vec<(usize, usize)> = line.split(" -> ").map(|s| {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }).collect();
        for (&(x1, y1), &(x2, y2)) in parts.iter().zip(parts.iter().skip(1)) {
            assert!(x1 == x2 || y1 == y2);
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    board[y * w + x] = b'#';
                }
            }
        }
    }
    let mut path = vec![(500, 0)];
    let mut cnt = 0;
    'outer: loop {
        loop {
            let (x, y) = *path.last().unwrap();
            if y + 1 == h {
                break 'outer;
            }
            let mut moved = false;
            for x2 in [x, x - 1, x + 1] {
                if board[(y + 1) * w + x2] == b'.' {
                    path.push((x2, y + 1));
                    moved = true;
                    break;
                }
            }
            if !moved {
                board[y * w + x] = b'o';
                cnt += 1;
                path.pop().unwrap();
            }
        }
    }
    out(cnt.to_string());
}
