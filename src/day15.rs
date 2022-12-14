fn parse_coords(s: &str) -> (i32, i32) {
    let s = s.strip_prefix("x=").unwrap();
    let (x, y) = s.split_once(", y=").unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut sensors: Vec<(i32, i32)> = vec![];
    let mut beacons: Vec<(i32, i32)> = vec![];
    for line in input.lines() {
        let line = line.strip_prefix("Sensor at ").unwrap();
        let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();
        sensors.push(parse_coords(sensor));
        beacons.push(parse_coords(beacon));
    }

    let mut range_ends: Vec<(i32, i16)> = vec![];
    let y = 2000000;
    let mut xpys = vec![];
    let mut xmys = vec![];
    for (beacon, sensor) in beacons.iter().zip(sensors.iter()) {
        let dist = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
        let dy = (y - sensor.1).abs();
        if dy <= dist {
            let x1 = sensor.0 - (dist - dy);
            let x2 = sensor.0 + (dist - dy);
            range_ends.push((x1, 1));
            range_ends.push((x2 + 1, -1));
        }
        xpys.push(sensor.0 + sensor.1 + dist + 1);
        xpys.push(sensor.0 + sensor.1 - dist - 1);
        xmys.push(sensor.0 - sensor.1 + dist + 1);
        xmys.push(sensor.0 - sensor.1 - dist - 1);
        if y == beacon.1 {
            range_ends.push((beacon.0, -1000));
            range_ends.push((beacon.0 + 1, 1000));
        }
    }
    range_ends.sort_unstable();
    let mut count = 0;
    let mut prev_x = range_ends[0].0;
    let mut part1 = 0;
    for &(x, d) in &range_ends {
        if count > 0 {
            part1 += x - prev_x;
        }
        count += d;
        prev_x = x;
    }
    out(part1.to_string());

    xpys.sort_unstable();
    xpys.dedup();
    xmys.sort_unstable();
    xmys.dedup();
    let n = 4000000;
    for &xpy in &xpys {
        for &xmy in &xmys {
            if (xpy + xmy) % 2 != 0 {
                continue;
            }
            let x = (xpy + xmy) / 2;
            let y = xpy - x;
            if !(0..=n).contains(&x) || !(0..=n).contains(&y) {
                continue;
            }
            let mut good = true;
            for (beacon, sensor) in beacons.iter().zip(sensors.iter()) {
                let dist = (beacon.0 - sensor.0).abs() + (beacon.1 - sensor.1).abs();
                if (sensor.0 - x).abs() + (sensor.1 - y).abs() <= dist {
                    good = false;
                    break;
                }
            }
            if good {
                out((x as i64 * n as i64 + y as i64).to_string());
            }
        }
    }
}
