pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut lines = input.lines();
    let mut sum = 0;
    for idx in 1.. {
        let v1 = Value::parse(lines.next().unwrap());
        let v2 = Value::parse(lines.next().unwrap());
        if v1 < v2 {
            sum += idx;
        }
        match lines.next() {
            None => break,
            Some("") => {}
            _ => panic!(),
        }
    }
    out(sum.to_string());
}

enum Value {
    Num(i32),
    List(Vec<Value>),
}

impl std::cmp::Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let (Value::Num(x), Value::Num(y)) = (self, other) {
            return x.cmp(y);
        }
        let xs = match self {
            Value::Num(_) => std::slice::from_ref(self),
            Value::List(xs) => xs,
        };
        let ys = match other {
            Value::Num(_) => std::slice::from_ref(other),
            Value::List(ys) => ys,
        };
        xs.cmp(ys)
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Value {}

impl Value {
    fn parse(mut s: &str) -> Self {
        let res = Self::parse_prefix(&mut s);
        assert_eq!(s, "");
        res
    }

    fn parse_prefix(s: &mut &str) -> Self {
        if try_consume_prefix("[", s) {
            let mut xs = vec![];
            if try_consume_prefix("]", s) {
                return Value::List(xs);
            }
            loop {
                xs.push(Self::parse_prefix(s));
                if try_consume_prefix("]", s) {
                    return Value::List(xs);
                }
                assert!(try_consume_prefix(",", s));
            }
        } else {
            let end = s.find(|c| c == ',' || c == ']').unwrap_or(s.len());
            let (num, rest) = s.split_at(end);
            *s = rest;
            Value::Num(num.parse().unwrap())
        }
    }
}

fn try_consume_prefix(prefix: &str, s: &mut &str) -> bool {
    if let Some(rest) = s.strip_prefix(prefix) {
        *s = rest;
        true
    } else {
        false
    }
}
