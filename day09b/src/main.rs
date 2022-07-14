use std::collections::BinaryHeap;


const DIR:[(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn dfs(mut map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    
    let m = map.len();
    let n = map[0].len();

    if x < n && y < m && map[y][x] != 9 {
        map[y][x] = 9;
        let mut sum = 1u32;
        for (xi, yi) in DIR {
            sum += dfs(&mut map, x.wrapping_add(xi as usize), y.wrapping_add(yi as usize));
        }
        sum
    } else {
        0
    }
}

fn main() {
    let mut map: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let m = map.len();
    let n = map[0].len();



    let mut heap = BinaryHeap::<u32>::new();
    for i in 0..m {
        for j in 0..n {
            let size = dfs(&mut map, j, i);
            if size > 0 {
                heap.push(size);
            }
        }
    }
    println!("{}", heap.pop().unwrap() * heap.pop().unwrap() * heap.pop().unwrap());
}
