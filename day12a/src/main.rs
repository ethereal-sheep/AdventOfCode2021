use std::collections::{HashMap, HashSet};

fn dfs(al: &HashMap<String, HashSet<String>>, mut vis: &mut HashSet<String>, curr: &str) -> i32 {
    if !vis.contains(curr) {
        if curr == "end" {
            return 1;
        }

        let mut sum = 0;
        if curr.chars().next().unwrap().is_lowercase() {
            vis.insert(curr.to_string());
        }

        for n in al.get(&curr.to_string()).unwrap() {
            sum += dfs(&al, &mut vis, &n);
        }
        if curr.chars().next().unwrap().is_lowercase() {
            vis.remove(&curr.to_string());
        }
        return sum;
    }
    0
}

fn main() {

    let mut al: HashMap<String, HashSet<String>> = HashMap::new();

    include_str!("../input.txt").lines().for_each(|l|{
        let (s, e) = l.split_once('-').unwrap();
        al.entry(s.to_string()).or_insert(HashSet::new()).insert(e.to_string());
        al.entry(e.to_string()).or_insert(HashSet::new()).insert(s.to_string());
    });

    println!("{}", dfs(&al, &mut HashSet::new(), "start"));

}
