fn main() {
    let mut ra = vec![0; 2000];
    for i in 1..2000 {
        ra[i] = i + ra[i - 1];
    }
    let v: Vec<i32> = include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let mut gs = usize::MAX;
    for i in 0..2000 {
        let mut sum = 0;

        for n in v.iter() {
            sum += ra[(i - *n).abs() as usize];
        }

        gs = std::cmp::min(sum, gs);
    }

    println!("{}", gs);
}
