pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut segments: Vec<(usize, usize, usize, usize)> = vec![];
    for line in input.lines() {
        let mut prev = None;
        for s in line.split(" -> ") {
            let (x, y) = s.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            if let Some((px, py)) = prev {
                segments.push((px, py, x, y));
            }
            prev = Some((x, y));
        }
    }
    let mut h = 0;
    for &(_x1, y1, _x2, y2) in &segments {
        h = h.max(y1).max(y2);
    }
    h += 3;
    assert!(h < 500);
    let w = 500 + h;
    let mut board = vec![b'.'; w * h];
    board[(h - 1) * w..].fill(b'#');
    for &(x1, y1, x2, y2) in &segments {
        assert!(x1 == x2 || y1 == y2);
        for x in x1.min(x2)..=x1.max(x2) {
            for y in y1.min(y2)..=y1.max(y2) {
                board[y * w + x] = b'#';
            }
        }
    }
    let mut path = vec![(500, 0)];
    let mut cnt = 0;
    let mut part1 = true;
    while !path.is_empty() {
        let (x, y) = *path.last().unwrap();
        let mut moved = false;
        for x2 in [x, x - 1, x + 1] {
            if board[(y + 1) * w + x2] == b'.' {
                path.push((x2, y + 1));
                moved = true;
                break;
            }
        }
        if !moved {
            if part1 && y + 2 == h {
                out(cnt.to_string());
                part1 = false;
            }
            board[y * w + x] = b'o';
            cnt += 1;
            path.pop().unwrap();
        }
    }
    out(cnt.to_string());
}
