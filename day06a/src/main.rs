use std::collections::VecDeque;

fn main() {
    let mut deq: VecDeque<usize> = VecDeque::from([0; 9]);
    include_str!("../input.txt")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| deq[x] += 1);

    (0..80).for_each(|_| {
        if let Some(f) = deq.pop_front() {
            deq.push_back(f);
            deq[6] += f;
        }
    });

    println!("{}", deq.iter().sum::<usize>());
}
