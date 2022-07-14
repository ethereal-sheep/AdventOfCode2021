fn main() {
    let s = include_str!("../input.txt");
    let numbers = s.lines().into_iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let mut ans = 0;
    for slice in numbers.windows(2) {
        match slice {
            [a, b] => {
                if a < b {
                    ans += 1;
                }
            },
            _ => ()
        }
    }
    

    
    print!("{}", ans);

}
