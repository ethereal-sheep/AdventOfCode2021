fn main() {
    let sum: i32 = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split_once(" | ")
                .unwrap()
                .1
                .split(' ')
                .map(|s| s.chars().filter(|c| *c >= 'a' && *c <= 'g').count())
                .map(|count| match count {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum();

    print!("{}", sum);
}
