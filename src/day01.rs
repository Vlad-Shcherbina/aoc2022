pub(crate) fn solve(input: &str, out: &mut dyn FnMut(String)) {
    let mut elves: Vec<i32> = input.split("\n\n")
        .map(|elf|
            elf.split_terminator('\n').map(
                |s| s.parse::<i32>().unwrap()).sum::<i32>())
        .collect();
    elves.sort();
    out(elves.last().unwrap().to_string());
    out(elves[elves.len() - 3..].iter().sum::<i32>().to_string());
}
