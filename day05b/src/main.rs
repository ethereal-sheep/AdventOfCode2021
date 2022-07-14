use std::mem::swap;

fn main() {
    let pts: Vec<((usize, usize), (usize, usize))> = include_str!("../input.txt")
        .lines()
        .map(|s| {
            let (p1, p2) = s.split_once(" -> ").unwrap();
            let map = |p: &str| -> (usize, usize) {
                let (x, y) = p.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            };
            (map(p1), map(p2))
        })
        .collect();

    let mut map = vec![vec![-1; 1000]; 1000];

    for ((mut x1, mut y1), (mut x2, mut y2)) in pts {
        if x1 == x2 {
            // differ on y
            if y1 > y2 {
                swap(&mut y1, &mut y2);
            }
            for i in y1..=y2 {
                map[x1][i] += 1;
            }
        } else if y1 == y2 {
            // differ on x
            if x1 > x2 {
                swap(&mut x1, &mut x2);
            }
            for i in x1..=x2 {
                map[i][y1] += 1;
            }
        } else {
            if x1 < x2 {
                if y1 < y2 {
                    let mut x = x1..=x2;
                    let mut y = y1..=y2;
                    while let (Some(xi), Some(yi)) = (x.next(), y.next()) {
                        map[xi][yi] += 1;
                    }
                } else {
                    let mut x = x1..=x2;
                    let mut y = (y2..=y1).rev();
                    while let (Some(xi), Some(yi)) = (x.next(), y.next()) {
                        map[xi][yi] += 1;
                    }
                }
            } else {
                if y1 < y2 {
                    let mut x = (x2..=x1).rev();
                    let mut y = y1..=y2;
                    while let (Some(xi), Some(yi)) = (x.next(), y.next()) {
                        map[xi][yi] += 1;
                    }
                } else {
                    let mut x = (x2..=x1).rev();
                    let mut y = (y2..=y1).rev();
                    while let (Some(xi), Some(yi)) = (x.next(), y.next()) {
                        map[xi][yi] += 1;
                    }
                }
            }
        }
    }
    let mut ans = 0;

    // for y in 0..10usize {
    //     for x in 0..10usize {
    //         if map[x][y] >= 0 {
    //             print!("{}", map[x][y]);
    //         } else {
    //             print!("{}", '.');
    //         }
    //     }
    //     println!();
    // }

    map.into_iter().for_each(|ref v| {
        v.iter().for_each(|i| {
            if *i > 0 {
                ans += 1
            }
        })
    });
    println!("{ans}");
}
