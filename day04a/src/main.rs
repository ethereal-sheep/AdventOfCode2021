use std::collections::HashMap;

const N: usize = 5;

struct Board {
    lookup: HashMap<i32, usize>,
    check: Vec<i32>
}

impl Board {
    pub fn new(lookup: HashMap<i32, usize>) -> Self {
        Board { lookup: lookup, check: vec![0; N * N] }
    }

    pub fn set(&mut self, n: i32) -> bool {
        
        if let Some(index) = self.lookup.get(&n) {

            let x = *index;
            // set the index
            self.check[x] = 1;
            // check if win
            // step 1: check horizontal
            let f = x - (x % N);
            if (f..f+N).all(|ix| self.check[ix] == 1) {
                println!("Horizontal: {}", n);
                return true;
            } 
            
            // step 2: check vertical
            let f = x % N;
            if (0..N).map(|i|i*N+f).all(|ix| self.check[ix] == 1) {
                println!("Vertical: {}", n);
                return true;
            }
            // // step 3: check left diagonal
            // if x % (N+1) == 0 && (0..N).map(|i| i*(N+1)).all(|ix| self.check[ix] == 1) {
            //     println!("Left Diagonal: {}", n);
            //     return true;
            // }
            // // step 4: check right diagonal
            // if x % (N-1) == 0 && (1..=N).map(|i| i*(N-1)).all(|ix| self.check[ix] == 1) {
            //     println!("Right Diagonal: {}", n);
            //     return true;
            // }
            

        }
        false
    }

    pub fn score(&self) -> i32 {
        
        (&self.lookup).into_iter()
        .filter(|&(_, ix)| self.check[*ix] == 0)
        .map(|(n, _)| n)
        .sum()
    }

}


fn main() {
    let (nums, boards) = include_str!("../input.txt")
    .split_once("\n\n")
    .unwrap();
    
    let ns = nums.split(",")
    .map(|s| s.parse::<i32>().unwrap())
    .into_iter();

    let mut bs: Vec<Board> = boards.split("\n\n")
    .map(|l| Board::new(
        l.split_ascii_whitespace()
        .enumerate()
        .map(|(i, n)| (n.parse::<i32>().unwrap(), i))
        .collect()
        )
    )
    .collect();

    for i in ns {
        for b in &mut bs {
            if b.set(i) {
                print!("Success! Score: {}", b.score() * i);
                return;
            }
        }
    }
}
