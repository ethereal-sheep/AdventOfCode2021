use std::collections::{HashMap, HashSet};

fn dfs(al: &HashMap<String, Vec<String>>, mut vis: &mut HashSet<String>, mut path: &mut Vec<String>, entered_small: bool, curr: &str) -> i32 {

    if !vis.contains(curr) {
        if curr == "end" {
            return 1;
        }

        let mut sum = 0;
        let is_lower = curr.chars().next().unwrap().is_lowercase();
        if is_lower {
            vis.insert(curr.to_string());
        }
        path.push(curr.to_string());
        for n in al.get(&curr.to_string()).unwrap() {
            sum += dfs(&al, &mut vis, &mut path, entered_small, &n);
        }
        path.pop();

        if is_lower {
            vis.remove(&curr.to_string());
        }
        return sum;
    } else if !entered_small && curr != "start" {
        if curr == "end" {
            println!("{:?}", path);
            return 1;
        }

        let mut sum = 0;
        path.push(curr.to_string());
        for n in al.get(&curr.to_string()).unwrap() {
            sum += dfs(&al, &mut vis, &mut path, true, &n);
        }
        path.pop();

        return sum;
    }
    0
}

fn main() {

    let mut al: HashMap<String, Vec<String>> = HashMap::new();

    include_str!("../input.txt").lines().for_each(|l|{
        let (s, e) = l.split_once('-').unwrap();
        al.entry(s.to_string()).or_insert(Vec::new()).push(e.to_string());
        al.entry(e.to_string()).or_insert(Vec::new()).push(s.to_string());
    });

    println!("{}", dfs(&al, &mut HashSet::new(), &mut vec![], false, "start"));

}
