use std::collections::HashSet;


enum Instruction {
    FoldX(i32),
    FoldY(i32),
}

impl Instruction {
    pub fn new(line: &str) -> Instruction {
        match line.split_once('=').unwrap() {
            ("fold along y", i) => Self::FoldY(i.parse().unwrap()),
            ("fold along x", i) => Self::FoldX(i.parse().unwrap()),
            _ => panic!()
        }
    }
}

struct Paper {
    coords: Vec<(i32, i32)>,
}

impl Paper {
    pub fn new(lines: &str) -> Paper {
        Paper {
            coords: lines
            .lines()
            .map(|l|l.split_once(",").unwrap())
            .map(|(x,y)| (x.parse().unwrap(), y.parse().unwrap()) )
            .collect()
        }
    }

    pub fn fold(&mut self, inst: Instruction) {

        match inst {
            Instruction::FoldX(i) => {
                self.coords
                .iter_mut()
                .for_each(|(x,_)| {
                    if *x > i {
                        *x = i + i - *x;
                    }
                });
            },
            Instruction::FoldY(i) => {
                self.coords
                .iter_mut()
                .for_each(|(_,y)| {
                    if *y > i {
                        *y = i + i - *y;
                    }
                });
            },
        }
    }

    pub fn count(&self) -> usize {
        let set: HashSet<(i32, i32)> = HashSet::from_iter(self.coords.iter().cloned());

        // for i in 0..20 {
        //     for j in 0..20 {
        //         print!("{}", if set.contains(&(j,i)){ '#' } else { '.' } );
        //     }
        //     println!();
        // }

        set.len()
    }
}

fn main() {
    let (coords, instructions) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let mut p = Paper::new(coords);

    p.fold(Instruction::new(instructions.lines().next().unwrap()));

    println!("{}", p.count());
}