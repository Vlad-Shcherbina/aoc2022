pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut names = vec![];
    let mut flow_rates = vec![];
    for line in input.lines() {
        let line = line.strip_prefix("Valve ").unwrap();
        let (name, rest) = line.split_once(" has flow rate=").unwrap();
        let flow_rate = rest.split_once(';').unwrap().0.parse::<i32>().unwrap();
        names.push(name);
        flow_rates.push(flow_rate);
    }
    let n = names.len();
    let mut dist = vec![30; n * n];
    for i in 0..n {
        dist[i * n + i] = 0;
    }
    for (i, line) in input.lines().enumerate() {
        let valves = line.split_once("tunnels lead to valves ")
            .xor(line.split_once("tunnel leads to valve "))
            .unwrap().1;
        for valve in valves.split(", ") {
            let j = names.iter().position(|&name| name == valve).unwrap();
            dist[i * n + j] = 1;
            dist[j * n + i] = 1;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let s = dist[i * n + k] + dist[k * n + j];
                if s < dist[i * n + j] {
                    dist[i * n + j] = s;
                }
            }
        }
    }
    let mut nonzero = vec![];
    for (i, &flow) in flow_rates.iter().enumerate() {
        if flow > 0 {
            nonzero.push(i);
        }
    }
    let mut best = vec![0i32; 1 << nonzero.len()];
    #[allow(clippy::too_many_arguments)]
    fn rec(cur: usize, t: i32, mask: usize, acc: i32, flow_rates: &[i32], dist: &[i32], nonzero: &[usize], best: &mut [i32]) {
        let n = flow_rates.len();
        for (i, &valve) in nonzero.iter().enumerate() {
            if (mask >> i) & 1 != 0 {
                continue;
            }
            let t2 = t - dist[cur * n + valve] - 1;
            if t2 <= 0 {
                continue;
            }
            let score = acc + t2 * flow_rates[valve];
            best[mask] = best[mask].max(score);
            rec(valve, t2, mask | (1 << i), score, flow_rates, dist, nonzero, best);
        }
    }
    let start = names.iter().position(|&name| name == "AA").unwrap();
    rec(start, 30, 0, 0, &flow_rates, &dist, &nonzero, &mut best);
    let part1 = best.iter().max().unwrap();
    out(part1.to_string());
}
