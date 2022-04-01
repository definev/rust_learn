use std::io::{stdin, BufRead};

///
/// 13214100000 -> 1.32141 x 10^10
///

fn main() {
    let stdin = stdin();
    let mut input_iterator = stdin.lock().lines();
    let input = input_iterator.next().unwrap().unwrap();
    let mut expo = 0;

    for last in input.as_bytes().into_iter().rev() {
        if last != &('0' as u8) {
            break;
        } else {
            expo = expo + 1;
        }
    }
    
    if let Some(new_format) = input.get(0..input.len() - expo) {
        expo = expo + new_format.len() - 1;
        let mut new_format = new_format.to_string();
        if new_format.len() > 1 {
            String::insert(&mut new_format, 1, '.');
        }
        println!("{new_format} x 10^{expo}");
    }
}
