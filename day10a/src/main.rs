use std::collections::HashMap;

fn main() {
    let pt_lookup = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let pair_lookup = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);

    let sum: i32 = include_str!("../input.txt")
        .lines()
        .filter_map(|l| {
            let mut stack = vec![' '; 0];
            for c in l.chars() {
                match c {
                '{'|'('|'['|'<' => stack.push(c),
                '}'|')'|']'|'>' => {
                    match (stack.last(), pair_lookup.get(&c)) {
                        (Some(t), Some(p)) if *t == *p => { stack.pop(); },
                        _ =>  return pt_lookup.get(&c).copied()
                    }
                },
                _ => panic!()
            }
            }
            None
        })
        .sum();

    println!("{}", sum);
}
