#[macro_use]
extern crate nom;

use nom::{IResult, digit};
use std::collections::BTreeSet;
use std::io::{self, Read};
use std::str;
use std::str::FromStr;

#[derive(Debug)]
enum Dir {
    Left,
    Right
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    dist: u16
}

named!(dir<Dir>, alt_complete!(value!(Dir::Left, char!('L')) | value!(Dir::Right, char!('R'))));

named!(step<Step>,
       chain!(
           d: dir ~
           s: map_res!(map_res!(digit, str::from_utf8), FromStr::from_str),
           || {Step{dir: d, dist: s}}
       )
);       

named!(input<Vec<Step> >,
       separated_nonempty_list!(
           tag!(", "),
           step
       )
);

#[derive(PartialEq, Eq, Debug)]
enum Compass {
    N,E,S,W
}

impl Compass {
    fn rotate(&self, way: &Dir) -> Compass {
        match *self {
            Compass::N => match *way {
                Dir::Left => Compass::W,
                Dir::Right => Compass::E
            },
            Compass::E => match *way {
                Dir::Left => Compass::N,
                Dir::Right => Compass::S
            },
            Compass::S => match *way {
                Dir::Left => Compass::E,
                Dir::Right => Compass::W
            },
            Compass::W => match *way {
                Dir::Left => Compass::S,
                Dir::Right => Compass::N
            }
        }
    }

    fn turn(&mut self, way: &Dir) {
        *self = self.rotate(way);
    }
    
    fn step_by(&self, coord: &mut (i16, i16)) {
        match *self {
            Compass::N => coord.1 += 1,
            Compass::E => coord.0 += 1,
            Compass::S => coord.1 -= 1,
            Compass::W => coord.0 -= 1
        };
    }
}

fn main() {
    let mut i : Vec<u8> = Vec::new();
    let _ = io::stdin().read_to_end(&mut i);
    let moves : Vec<Step> = match input(i.as_slice()) {
        IResult::Done(_, r) => r,
        _ => panic!()
    };
    let mut dir = Compass::N;
    let mut loc = (0i16, 0i16);
    let mut visited = BTreeSet::new();
    let mut found = false;
    for Step{dir: d, dist: s} in moves {
        dir.turn(&d);
        for _ in 0..s {
            dir.step_by(&mut loc);
            if !found {
                if visited.contains(&loc) {
                    println!("Repeated visit to {}, {}, {}", loc.0, loc.1, loc.0.abs() + loc.1.abs());
                    found = true;
                }
                visited.insert(loc);
            }
        }
    }
    println!("Ended up at {}, {}, {}", loc.0, loc.1, loc.0.abs() + loc.1.abs());
}
