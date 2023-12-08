use std::cmp::Reverse;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use core::cmp::Ordering;


struct Hand{
    hand: String,
    bid: usize,
    value: u32
}

pub fn task() {
    parse_file();
}


fn parse_file(){
    // Create a path to the desired file
    let path = Path::new("./src/day_seven/input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("\nwinnings:{}", get_winnings(s)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_winnings(input: String) -> usize {

    let lines: Vec<_> = input.split("\r\n").collect();
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| {
            let l:Vec<_> = line.split(" ").collect();
            let cards = convert(l[0]);
            Hand{ hand: cards.clone() , bid: l[1].parse::<usize>().unwrap(), value: get_hand_value(cards)}
        })
        .collect();


    hands.sort_by(|a, b|{

        if a.value > b.value {
             Ordering::Greater
        } else if a.value < b.value {
            Ordering::Less
        } else{
            let mut x = 0;
            let mut ordering = Ordering::Equal;
            let ac = a.hand.chars().collect::<Vec<_>>();
            let bc = b.hand.chars().collect::<Vec<_>>();
            while x < 5 {
                if ac[x] < bc[x] { 
                    ordering = Ordering::Less;
                    break;
                }
                else if ac[x] > bc[x] { 
                    ordering = Ordering::Greater ;
                    break;
                }
                x += 1;
            }
            ordering
        }

    });

    let mut x = 0;
    let mut sum: usize = 0;
    while x < hands.len(){
        sum += (x+1) * hands[x].bid;
        x += 1;
    }
    return sum;
}

fn convert(s: &str) -> String {
    s.chars().map(|c| {
    match c {
        '2' => 'A',
        '3' => 'B',
        '4' => 'C',
        '5' => 'D',
        '6' => 'E',
        '7' => 'F',
        '8' => 'G',
        '9' => 'H',
        'T' => 'I',
        'J' => 'J',
        'Q' => 'K',
        'K' => 'L',
        'A' => 'M',
         _ => 'Z'
    }}).collect::<String>()
}

fn get_hand_value(hand: String) -> u32 {
    let mut s = hand.chars().collect::<Vec<_>>();
    s.sort();
    let c = s.iter().collect::<String>();
    let mut cards: &str = &c;

    //https://stackoverflow.com/questions/49099560/how-to-split-string-into-units-of-each-character

    fn first_different(mut chars: std::str::Chars) -> Option<usize> {
        chars.next().map(|f| chars.take_while(|&c| c == f).fold(f.len_utf8(), |len, c| len + c.len_utf8()))
    }

    let mut output = Vec::new();

    while let Some(different) = first_different(cards.chars()) {
        let (before, after) = cards.split_at(different);
        cards = after;
        output.push(before);
    }

    output.sort_by_key(|g| Reverse(g.len()));

    match output.len(){
        1 => 6, //five of a kind
        2 => { 
            match output[0].len(){
                3 => 4, // full house
                4 => 5, // four of a kind
                _ => 0
            }
        },
        3 => {
            match output[0].len(){
                3 => 3, // three of a kind
                2 => 2, // two pair
                _ => 0
            }
        },
        4 => 1, // one pair
        _ => 0, // high card
    }

}