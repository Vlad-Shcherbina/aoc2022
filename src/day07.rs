use fxhash::FxHashMap as HashMap;

pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut dir_sizes: HashMap<String, i32> = HashMap::default();

    let mut cwd = None;
    let mut lines = input.split_terminator('\n').peekable();
    while let Some(cmd) = lines.next() {
        let cmd = cmd.strip_prefix("$ ").unwrap();
        if let Some(arg) = cmd.strip_prefix("cd ") {
            match arg {
                "/" => cwd = Some("/".to_owned()),
                ".." => {
                    let cwd = cwd.as_mut().unwrap();
                    cwd.truncate(cwd[..cwd.len() - 1].rfind('/').unwrap() + 1);
                }
                _ => {
                    let cwd = cwd.as_mut().unwrap();
                    cwd.push_str(arg);
                    cwd.push('/');
                }
            }
        } else {
            assert_eq!(cmd, "ls");
            let mut total_size = 0;
            while lines.peek().map_or(false, |s| !s.starts_with("$ ")) {
                let line = lines.next().unwrap();
                let (size, _name) = line.split_once(' ').unwrap();
                if size != "dir" {
                    total_size += size.parse::<i32>().unwrap();
                }
            }
            let mut d = cwd.as_deref().unwrap();
            loop {
                *dir_sizes.entry(d.to_owned()).or_default() += total_size;
                if d == "/" {
                    break;
                }
                d = &d[..d[..d.len() - 1].rfind('/').unwrap() + 1];
            }
        }
    }
    let mut result = 0;
    for &size in dir_sizes.values() {
        if size <= 100_000 {
            result += size;
        }
    }
    out(result.to_string());
    let total = dir_sizes["/"];
    let mut sizes: Vec<i32> = dir_sizes.values().copied().collect();
    sizes.sort_unstable();
    for size in sizes {
        if total - size <= 70_000_000 - 30_000_000 {
            out(size.to_string());
            break;
        }
    }
}
