fn main() {
    let s = include_str!("../input.txt");

    let mut count =  vec![0; 12];
    for line in s.lines() {
        for (i, c) in line.chars().enumerate() {
            count[i] += if c == '1' { 1 } else { -1 }
        }
    }
    let mut gamma = 0u64;
    let mut epsilon =  0u64;
    
    for i in count {
        gamma <<= 1;
        epsilon <<= 1;
        if i > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("{}", gamma);
    println!("{}", epsilon);
    println!("{}", gamma * epsilon);


}
