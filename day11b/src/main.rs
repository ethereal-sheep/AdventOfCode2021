const DIR: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, -1), (-1, 1), (1, -1)];

fn dfs2(mut map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let m = map.len();
    let n = map[0].len();

    if x < n && y < m && map[y][x] != 0 {
        map[y][x] += 1;
        if map[y][x] >= 10 {
            map[y][x] = 0;
            let mut sum = 1u32;
            for (xi, yi) in DIR {
                sum += dfs2(
                    &mut map,
                    x.wrapping_add(xi as usize),
                    y.wrapping_add(yi as usize),
                );
            }
            return sum;
        }
    }
    0
}

fn dfs(mut map: &mut Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let m = map.len();
    let n = map[0].len();

    if x < n && y < m {
        if map[y][x] >= 10 {
            map[y][x] = 0;
            let mut sum = 1u32;
            for (xi, yi) in DIR {
                sum += dfs2(
                    &mut map,
                    x.wrapping_add(xi as usize),
                    y.wrapping_add(yi as usize),
                );
            }
            return sum;
        }
    }
    0
}

fn main() {
    let mut map: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|i| i.to_digit(10).unwrap()).collect())
        .collect();

    let m = map.len();
    let n = map[0].len();


    for step in 1.. {
        let mut flash = 0u32;
        for i in 0..m {
            for j in 0..n {
                map[i][j] += 1;
            }
        }
        for i in 0..m {
            for j in 0..n {
                flash += dfs(&mut map, j, i);
            }
        }
        if flash as usize == n * m {
            println!("{step}");
            break;
        }
    }


}
