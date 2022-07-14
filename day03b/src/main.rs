fn find(nums: &Vec<u32>, find_most: bool) -> u32 {

    let mut ol = 0;
    let mut or = (nums.len()-1) as i32;

    let byte_len = 12;
    let mut checking =  1 << (byte_len - 1);

    for i in 0..byte_len {
        if or == ol {
            break;
        }
        let mut l = ol;
        let mut r = or;
        let mut closest_zero: Option<i32> = None;
        // find the first with checking bit not set
        while l <= r {
            let m = l + (r-l) / 2;
            if nums[m as usize] < checking {
                // if less than ^, means nums[m] has the checking bit not-set
                // else move to right
                closest_zero = Some(m);
                l = m+1;

            } else {
                // move to the left
                r = m-1;
            }
        }
        // nums[closest_zero] is the first zero
        // so l - ol is the number of nums with checking bit not-set
        // and or - l + 1 is number of nums with checking bit set
        if find_most {
            match closest_zero {
                None => (),
                Some(x) => {
                    if x - ol + 1 < or - x {
                        // more ones than zeros
                        ol = x + 1;
                        if or == ol {
                            break;
                        }
                        checking += 1 << (byte_len - i - 2);
    
                    } else if x - ol + 1 > or - x {
                        // more zeros than ones
                        or = x;
                        if or == ol {
                            break;
                        }
                        checking -= 1 << (byte_len - i - 2);
                    } else {
                        // equal ones and zeros
                        ol = x + 1;
                        if or == ol {
                            break;
                        }
                        checking += 1 << (byte_len - i - 2);
                    }
                },
            }
        } else {
            match closest_zero {
                None => panic!("IMPOSSIBLE"),
                Some(x) => {
                    if x - ol + 1 > or - x {
                        // less ones than zeros
                        ol = x + 1;
                        if or == ol {
                            break;
                        }
                        checking += 1 << (byte_len - i - 2);
    
                    } else if x - ol + 1 > or - x {
                        // less zeros than ones
                        or = x;
                        if or == ol {
                            break;
                        }
                        checking -= 1 << (byte_len - i - 2);
                    } else {
                        // equal ones and zeros
                        or = x;
                        if or == ol {
                            break;
                        }
                        checking -= 1 << (byte_len - i - 2);
                    }
                },
            }
        }
    }
    nums[ol as usize]
}


fn main() {
    let s = include_str!("../input.txt");

    let lines = s.lines();
    let mut nums: Vec<u32> = lines
                            .map(|s| u32::from_str_radix(s, 2).unwrap())
                            .collect();
    
    nums.sort();
    
    let o2 = find(&nums, true);
    let co2 = find(&nums, false);

    println!("{}\n", o2 * co2);



}
