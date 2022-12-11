pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let width = input.find('\n').unwrap();
    let height = input.chars().filter(|&c| c == '\n').count();
    let input = input.as_bytes();

    let mut visible: Vec<bool> = vec![false; width * height];
    for i in 0..height {
        let mut j = 0;
        let mut cur_height = 0;
        while j < width {
            let h = input[i * (width + 1) + j] - b'0' + 1;
            if h > cur_height {
                visible[i * width + j] = true;
                cur_height = h;
            }
            j += 1;
        }
        let mut j = width - 1;
        let mut cur_height = 0;
        loop {
            let h = input[i * (width + 1) + j] - b'0' + 1;
            if h > cur_height {
                visible[i * width + j] = true;
                cur_height = h;
            }
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
    for j in 0..width {
        let mut i = 0;
        let mut cur_height = 0;
        while i < height {
            let h = input[i * (width + 1) + j] - b'0' + 1;
            if h > cur_height {
                visible[i * width + j] = true;
                cur_height = h;
            }
            i += 1;
        }
        let mut i = height - 1;
        let mut cur_height = 0;
        loop {
            let h = input[i * (width + 1) + j] - b'0' + 1;
            if h > cur_height {
                visible[i * width + j] = true;
                cur_height = h;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
    let result = visible.iter().filter(|&&b| b).count();
    out(result.to_string());
}
