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
    fn rec(cur: usize, t: i32, open: &mut [bool], flow_rates: &[i32], dist: &[i32]) -> i32 {
        let n = open.len();
        let mut best = 0;
        for i in 0..n {
            if open[i] || flow_rates[i] == 0 {
                continue;
            }
            let t2 = t - dist[cur * n + i] - 1;
            if t2 <= 0 {
                continue;
            }
            open[i] = true;
            let q = rec(i, t2, open, flow_rates, dist) + t2 * flow_rates[i];
            best = best.max(q);
            open[i] = false;
        }
        best
    }
    let start = names.iter().position(|&name| name == "AA").unwrap();
    let mut open = vec![false; n];
    let res = rec(start, 30, &mut open, &flow_rates, &dist);
    out(res.to_string());
}
