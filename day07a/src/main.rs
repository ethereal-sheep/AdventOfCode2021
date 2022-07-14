fn main() {
    let mut v = vec![0; 2000];
    let mut sum = 0;
    include_str!("../input.txt")
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|x| {
            v[x] += 1;
            sum += x as i32;
        });

    let mut psl = vec![0; 2000];
    psl[0] = v[0];
    for i in 1..2000usize {
        psl[i] = psl[i - 1] + v[i];
    }
    let mut rsl = vec![0; 2000];
    rsl[1999] = v[1999];
    for i in (0..1999usize).rev() {
        rsl[i] = rsl[i + 1] + v[i];
    }

    let mut gs = i32::MAX;
    let mut ls = sum;

    for i in 1..2000usize {
        ls = ls + psl[i - 1] - rsl[i];
        gs = std::cmp::min(ls, gs);
    }

    println!("{}", gs);
}
