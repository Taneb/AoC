use std::io::{stdin, BufRead, BufReader};

enum Dir {
    U, R, D, L
}

struct Code {
    code: u8
}

impl Code {
    fn update1(&self, dir: &Dir) -> Self {
        match *dir {
            Dir::U => match self.code {
                1...3 => Code{code : self.code},
                _ => Code{code : self.code - 3}
            },
            Dir::R => match self.code {
                3|6|9 => Code{code : self.code},
                _ => Code{code : self.code + 1}
            },
            Dir::D => match self.code {
                7...9 => Code{code : self.code},
                _ => Code{code : self.code + 3}
            },
            Dir::L => match self.code {
                1|4|7 => Code{code : self.code},
                _ => Code{code : self.code - 1}
            }
        }
    }

    fn update2(&self, dir: &Dir) -> Self {
        match *dir {
            Dir::U => match self.code {
                3|13 => Code{code: self.code - 2},
                6|7|8|10|11|12 => Code{code: self.code - 4},
                _ => Code{code: self.code}
            },
            Dir::L => match self.code {
                1|2|5|10|13 => Code{code: self.code},
                _ => Code{code: self.code - 1}
            },
            Dir::D => match self.code {
                1|11 => Code{code: self.code + 2},
                2|3|4|6|7|8 => Code{code: self.code + 4},
                _ => Code{code: self.code}
            },
            Dir::R => match self.code {
                1|4|9|12|13 => Code{code: self.code},
                _ => Code{code: self.code + 1}
            }
        }
    }
}

fn parse_char(c: &char) -> Dir {
    match *c {
        'U' => Dir::U,
        'R' => Dir::R,
        'D' => Dir::D,
        'L' => Dir::L,
        _ => panic!()
    }
}

fn main() {
    let mut code1 = Code{code: 5};
    let mut code2 = Code{code: 5};
    let mut out1 = String::new();
    let mut out2 = String::new();
    for line in BufReader::new(stdin()).lines() {
        for dir in line.unwrap().chars().map(|c| parse_char(&c)) {
            code1 = code1.update1(&dir);
            code2 = code2.update2(&dir);
        }
        out1.push_str(format!("{}", code1.code).as_str());
        out2.push_str(format!("{:X}", code2.code).as_str());
    }
    println!("The answer to the first part is {}", out1);
    println!("The answer to the second part is {}", out2);
}
