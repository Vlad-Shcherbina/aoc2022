pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let w = input.find('\n').unwrap() + 1;
    let h = input.lines().count();
    assert_eq!(input.len(), w * h);

    let start = input.find('S').unwrap();
    let end = input.find('E').unwrap();
    let (end_x, end_y) = (end % w, end / w);

    let mut input = input.as_bytes().to_owned();
    input[start] = b'a';
    input[end] = b'z';
    let mut visited = vec![false; input.len()];

    let mut frontier = vec![(end_x, end_y)];
    visited[end] = true;
    let mut dist = 0;
    let mut part2 = None;
    while !frontier.is_empty() {
        let mut new_frontier = vec![];
        for (x, y) in frontier {
            let i = y * w + x;
            if i == start {
                out(dist.to_string());
            }
            let cur = input[i];
            if cur == b'a' && part2.is_none() {
                part2 = Some(dist);
            }
            if x > 0 && input[i - 1] >= cur - 1 && !visited[i - 1] {
                visited[i - 1] = true;
                new_frontier.push((x - 1, y));
            }
            if x + 1 < w - 1 && input[i + 1] >= cur - 1 && !visited[i + 1] {
                visited[i + 1] = true;
                new_frontier.push((x + 1, y));
            }
            if y > 0 && input[i - w] >= cur - 1 && !visited[i - w] {
                visited[i - w] = true;
                new_frontier.push((x, y - 1));
            }
            if y + 1 < h && input[i + w] >= cur - 1 && !visited[i + w] {
                visited[i + w] = true;
                new_frontier.push((x, y + 1));
            }
        }
        frontier = new_frontier;
        dist += 1;
    }
    out(part2.unwrap().to_string());
}
