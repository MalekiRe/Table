mod parser;

use std::ops::Range;
use terminal_emoji::Emoji;
//
// mod parser;

fn main() {
    print_hearts(vec![3..7, 14..18]);
    print_hearts(vec![2..8, 13..19]);
    print_hearts(vec![1..9, 12..20]);
    for i in 0..3 {
        print_hearts(vec![0..21]);
    }
    for i in 1..11 {
        print_hearts(vec![i..21-i]);
    }


}
fn print_hearts(ranges: Vec<Range<u32>>) {
    for i in 0..21 {
        let mut did = false;
        for range in &ranges {
            if range.contains(&i) {
                heart();
                did = true;
            }
        }
        if !did {
            space()
        }
    }
    print!("\n");
}
fn space() {
    print!("🦀a");
}
fn heart() {
    print!("❤️");
}