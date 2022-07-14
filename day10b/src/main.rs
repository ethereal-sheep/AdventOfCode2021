use std::collections::HashMap;

fn main() {
    let pt_lookup = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let pair_lookup = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let mut sums: Vec<u64> = include_str!("../input.txt")
        .lines()
        .filter_map(|l| {
            let mut stack = vec![' '; 0];
            for c in l.chars() {
                match c {
                    '{' | '(' | '[' | '<' => stack.push(c),
                    '}' | ')' | ']' | '>' => match (stack.last(), pair_lookup.get(&c)) {
                        (Some(t), Some(p)) if *t == *p => {
                            stack.pop();
                        }
                        _ => return None,
                    },
                    _ => panic!(),
                }
            }

            let mut score: u64 = 0;
            while let Some(k) = stack.pop() {
                score *= 5;
                score += pt_lookup.get(&k).copied().unwrap();
            }
            Some(score)
        })
        .collect();
    
    sums.sort();
    println!("{}", sums[sums.len() / 2 as usize]);
}
