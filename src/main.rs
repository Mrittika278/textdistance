use textdistance_rs::edit::Levenshtein;
use textdistance_rs::Distance;

fn main() {
    let lev = Levenshtein;

    println!("{}", lev.distance("book", "back"));
    println!("{}", lev.distance("hello", "hello"));
    println!("{}", lev.distance("", "rust"));
}