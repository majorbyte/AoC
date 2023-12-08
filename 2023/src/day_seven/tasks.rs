use std::cmp::Reverse;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use core::cmp::Ordering;

#[derive(Eq)]
struct Hand{
    hand: String,
    bid: usize,
    value: u32
}

impl Hand {
    fn new(line: &str, t: &bool) -> Hand{
        let l:Vec<_> = line.split(" ").collect();
        Hand{ hand: convert(l[0], t), bid: l[1].parse::<usize>().unwrap(), value: get_hand_value(l[0], t)}
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value > other.value {
            Ordering::Greater
        } else if self.value < other.value {
            Ordering::Less
        } else{
            if self.hand < other.hand {
                Ordering::Less
            } else if self.hand > other.hand {
                Ordering::Greater
            } else{
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
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
        Ok(_) => print!("\nwinnings part 1:{}\nwinnings part 2:{}", get_winnings(&s, &false), get_winnings(&s, &true)),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed    
}

fn get_winnings(input: &str, t:  &bool) -> usize {
    let lines: Vec<_> = input.split("\r\n").collect();
    let mut hands: Vec<Hand> = lines
        .iter()
        .map(|line| Hand::new(line, t))
        .collect();

    hands.sort_by(|a, b| a.cmp(b));

    return hands.iter().enumerate().fold(0, |sum, (i, hand)| sum + (i+1) * hand.bid);
}

fn convert(s: &str, is_task2: &bool) -> String {
    s.chars().map(|c| {
    match c {
        '2' => 'B',
        '3' => 'C',
        '4' => 'D',
        '5' => 'E',
        '6' => 'F',
        '7' => 'G',
        '8' => 'H',
        '9' => 'I',
        'T' => 'J',
        'J' => match is_task2 {
            false => 'K',
            true => 'A',
        }
        'Q' => 'L',
        'K' => 'M',
        'A' => 'N',
         _ => 'Z'
    }}).collect::<String>()
}

fn get_hand_value(hand: &str, is_task2: &bool) -> u32 {
    let offset = hand.matches("J").count();
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

    
    if offset > 0  && *is_task2 {
        let mut y = 0;
        let mut biggest_group = output[0].len()+ offset;
        while y < output.len()-1 && output[y].contains("J"){
            y += 1;
            biggest_group = output[y].len()+ offset;
        }

        if biggest_group > 5 { biggest_group = 5;}
    
        match biggest_group{
            5 => 12, // five of a kind
            4 => 10, // four of a kind 
            3 =>  {  
                match output[1].len(){
                    2 => 8, // full house
                    _ => 6, // three of a kind
                }
            }, 
            2 => 2, // one pair
            _ => 0, // high card
        }
    } else {
        match output.len(){
            1 => 12, //five of a kind
            2 => { 
                match output[0].len(){
                    3 => 8, // full house
                    4 => 10, // four of a kind
                    _ => 0
                }
            },
            3 => {
                match output[0].len(){
                    3 => 6, // three of a kind
                    2 => 4, // two pair
                    _ => 0
                }
            },
            4 => 2, // one pair
            _ => 0, // high card
        }
    }
}