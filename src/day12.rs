pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let w = input.find('\n').unwrap() + 1;
    let h = input.lines().count();
    assert_eq!(input.len(), w * h);

    let start = input.find('S').unwrap();
    let (start_x, start_y) = (start % w, start / w);
    let end = input.find('E').unwrap();

    let mut input = input.as_bytes().to_owned();
    input[start] = b'a';
    input[end] = b'z';
    let mut visited = vec![false; input.len()];

    let mut frontier = vec![(start_x, start_y)];
    visited[start] = true;
    let mut dist = 0;
    loop {
        let mut new_frontier = vec![];
        for (x, y) in frontier {
            let i = y * w + x;
            if i == end {
                out(dist.to_string());
                return;
            }
            let cur = input[i];
            if x > 0 && input[i - 1] <= cur + 1 && !visited[i - 1] {
                visited[i - 1] = true;
                new_frontier.push((x - 1, y));
            }
            if x + 1 < w - 1 && input[i + 1] <= cur + 1 && !visited[i + 1] {
                visited[i + 1] = true;
                new_frontier.push((x + 1, y));
            }
            if y > 0 && input[i - w] <= cur + 1 && !visited[i - w] {
                visited[i - w] = true;
                new_frontier.push((x, y - 1));
            }
            if y + 1 < h && input[i + w] <= cur + 1 && !visited[i + w] {
                visited[i + w] = true;
                new_frontier.push((x, y + 1));
            }
        }
        frontier = new_frontier;
        dist += 1;
    }
}
