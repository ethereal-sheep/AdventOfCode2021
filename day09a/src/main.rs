fn main() {
    let map: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let m = map.len();
    let n = map[0].len();

    let check = |x: usize, y: usize, xi: i32, yi: i32| {
        let xn = x.wrapping_add(xi as usize);
        let yn = y.wrapping_add(yi as usize);
        // println!("{:?}", (xn, yn));
        if xn < n && yn < m {
            return Some(map[y][x] < map[yn][xn]);
        }
        None
    };
    let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut ans = 0;
    for i in 0..m {
        for j in 0..n {
            let mut lowest = true;
            for (xi, yi) in dir {
                if let Some(b) = check(j, i, xi, yi) {
                    lowest &= b;
                }
            }

            if lowest {
                ans += map[i][j] + 1;
            }
        }
    }

    println!("{}", ans);
}
