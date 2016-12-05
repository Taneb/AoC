#[macro_use]
extern crate nom;

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::io::{stdin, BufRead, BufReader};
use std::str::{self, FromStr};

use nom::{IResult, alpha, digit};

#[derive(PartialEq, Eq)]
struct Down<T> {
    up: T
}

fn order_rev(order: Ordering) -> Ordering {
    match order {
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
        Ordering::Greater => Ordering::Less
    }
}

impl<T: PartialOrd> PartialOrd for Down<T> {
    fn partial_cmp(&self, other: &Down<T>) -> Option<Ordering> {
        self.up.partial_cmp(&other.up).map(order_rev)
    }
}

impl<T: Ord> Ord for Down<T> {
    fn cmp(&self, other: &Down<T>) -> Ordering {
        order_rev(self.up.cmp(&other.up))
    }
}

struct RoomCode {
    name: Vec<String>,
    sector_id: usize,
    checksum: String
}



named!(
    chunk<String>,
    map!(
        map_res!(
            alpha,
            str::from_utf8
        ),
        String::from
    )
);

named!(
    chunks < Vec < String > >,
    separated_list!(tag!("-"),chunk)
);

named!(
    code<usize>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(
    checksum< String >,
    map!(
        delimited!(
            tag!("["),
            take_str!(5),
            tag!("]")
        ),
        String::from
    )
);

named!(
    room<RoomCode>,
    chain!(
        name: chunks ~
            tag!("-") ~
            sector: code ~
            check: checksum,
        ||{RoomCode{name: name, sector_id: sector, checksum: check}}
    )
);

impl RoomCode {
    fn check(&self) -> bool {
        let mut count : BTreeMap<char, usize> = BTreeMap::new();
        for part in self.name.iter() {
            for letter in part.chars() {
                *count.entry(letter).or_insert(0) += 1;
            }
        }
        let mut data : Vec<(&char, &usize)> = count.iter().collect();
        data.sort_by_key(|kv| kv.0);
        data.sort_by_key(|kv| Down{up: kv.1});
        data.iter().map(|kv| kv.0.clone()).take(5).eq(self.checksum.chars())
    }

    fn decrypt(&self) -> Vec<String> {
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let mut answer = Vec::with_capacity(self.name.len());
        let shift = self.sector_id % 26;
        for part in self.name.iter() {
            let mut answer_part = String::with_capacity(part.len());
            for c in part.chars() {
                let i = (alphabet.chars().position(|b| c == b).unwrap() + shift) % 26;
                answer_part.push(alphabet.chars().nth(i).unwrap());
                
            }
            answer.push(answer_part);
        }
        return answer;
    }
}



fn main() {
    let mut count = 0;
    for line in BufReader::new(stdin()).lines() {
        let room_code = match room(line.unwrap().into_bytes().as_slice()) {
            IResult::Done(_, o) => o,
            _ => panic!()
        };
        if room_code.check() {
            count += room_code.sector_id;
            if room_code.decrypt()[0] == "northpole" {
                println!("The room where the crap is stored is {}", room_code.sector_id)
            }
        }
    };
    println!("The total of the valid sector ids is {}", count);
}
