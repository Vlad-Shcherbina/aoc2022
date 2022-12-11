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
    let mut best_scenic_score = 0;
    for i in 1 .. height - 1 {
        for j in 1 .. width - 1 {
            let mut scenic_score = 1;
            let cur_height = input[i * (width + 1) + j];
            let mut d = 1;
            while d < j && input[i * (width + 1) + j - d] < cur_height {
                d += 1;
            }
            scenic_score *= d;
            let mut d = 1;
            while j + d + 1 < width && input[i * (width + 1) + j + d] < cur_height {
                d += 1;
            }
            scenic_score *= d;
            let mut d = 1;
            while d < i && input[(i - d) * (width + 1) + j] < cur_height {
                d += 1;
            }
            scenic_score *= d;
            let mut d = 1;
            while i + d + 1 < height && input[(i + d) * (width + 1) + j] < cur_height {
                d += 1;
            }
            scenic_score *= d;
            best_scenic_score = best_scenic_score.max(scenic_score);
        }
    }
    out(best_scenic_score.to_string());
}
