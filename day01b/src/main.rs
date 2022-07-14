
fn main() {
    let s = include_str!("../input.txt");
    let nums = s.lines().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    
    if nums.len() < 3 {
        return ;
    }

    let mut sums: Vec<i32> = vec![0; nums.len()-2];
    sums.push(nums[0..2].into_iter().sum());
    // create sums
    for i in 1..nums.len()-2 {
        sums[i] = sums[i-1] - nums[i-1] + nums[i+2];
    }
    let mut ans = 0;
    for slice in sums.windows(2) {
        match slice {
            [a,b] => {
                if a < b {
                    ans += 1;
                }
            },
            _ => ()
        }
    }

    print!("{}", ans);




}
