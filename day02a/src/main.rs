
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    pub fn new(s: &str) -> Self {
        let v: Vec<&str> = s.split(' ').collect();
        match v.as_slice() {
            [s, i] => {
                let val: i32 = i.parse().unwrap();
                match *s {
                    "forward" => return Command::Forward(val),
                    "up" => return Command::Up(val),
                    "down" => return Command::Down(val),
                    _ => panic!("no command found")
                };
            },
            _ => panic!("bad str: {:?}", s)
        }
    }
}

fn do_command(h: &mut i32, d: &mut i32, command: &str) {
    match Command::new(command) {
        Command::Forward(i) => *h += i,
        Command::Up(i) => *d -= i,
        Command::Down(i) => *d += i,
    }


}

fn main() {
    let lines = include_str!("../input.txt").lines();
    let mut h = 0;
    let mut d = 0;
    for line in lines.into_iter() {
        do_command(&mut h, &mut d, line);
    }

    print!("{}", h * d);
}
