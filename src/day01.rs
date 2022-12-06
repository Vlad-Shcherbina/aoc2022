pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let part1 = input.split("\n\n")
        .map(|elf|
            elf.split_terminator('\n').map(
                |s| s.parse::<i32>().unwrap()).sum::<i32>())
        .max().unwrap();
    out(part1.to_string());
}
