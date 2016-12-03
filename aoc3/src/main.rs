#[macro_use]
extern crate nom;

use nom::{IResult, digit};
use std::io::{stdin, BufRead, BufReader};
use std::mem::swap;
use std::str;
use std::str::FromStr;

named!(numberp<usize>, map_res!(
    map_res!(
        ws!(digit),
        str::from_utf8
    ),
    FromStr::from_str
));

named!(linep<(usize, usize, usize)>, tuple!(
    numberp, numberp, numberp
));

fn is_triangle(tup: &(usize, usize, usize)) -> bool {
    if tup.0 <= tup.2 {
        if tup.1 <= tup.2 {
            tup.0 + tup.1 > tup.2
        } else {
            tup.0 + tup.2 > tup.1
        }
    } else {
        if tup.0 <= tup.1 {
            tup.0 + tup.2 > tup.1
        } else {
            tup.1 + tup.2 > tup.0
        }
    }   
}

fn transpose<T>(tup : &mut ((T,T,T),(T,T,T),(T,T,T))) {
    swap(&mut (tup.1).0, &mut (tup.0).1);
    swap(&mut (tup.2).0, &mut (tup.0).2);
    swap(&mut (tup.1).2, &mut (tup.2).1);
}

fn main() {
    let mut count_hor = 0;
    let mut count_ver = 0;
    let mut cache = ((0,0,0),(0,0,0),(0,0,0));
    let mut cachen = 0;
    for line in BufReader::new(stdin()).lines() {
        let line = line.unwrap();
        let parsed = linep(line.as_bytes());
        let parsed = match parsed {
            IResult::Done(_, o) => o,
            _ => panic!()
        };
        if is_triangle(&parsed) {
            count_hor += 1;
        }
        cachen += 1;
        match cachen {
            1 => cache.0 = parsed,
            2 => cache.1 = parsed,
            _ => {
                cachen = 0;
                cache.2 = parsed;
                transpose(&mut cache);
                if is_triangle(&cache.0) {
                    count_ver += 1;
                }
                if is_triangle(&cache.1) {
                    count_ver += 1;
                }
                if is_triangle(&cache.2) {
                    count_ver += 1;
                }
            }
        }
    }
    println!("Triangles when reading horizontally: {}", count_hor);
    println!("Triangles when reading vertically: {}", count_ver);
}
