use std::panic::catch_unwind;

fn main() {
    let mut s : String = String::from("Hello Rust.");
    let mut s_slice : String = String::from("Hello Rust.");
    let index = first_word(&s);
    s.clear(); // TODO: cause async
    println!("The first word in s:\"{}\" is at index {} ", s, index);

    let str = "Hi boy."; //Todo: Type &str
    let first_word = _first_word(&s_slice);
    //s_slice.clear(); //cannot borrow `s_slice` as mutable because it is also borrowed as immutable
    println!("The first word in s_slice:\"{}\" is {} ", s_slice, first_word);
}

// slice
fn first_word(str: &String) -> usize{
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    str.len()
}

fn _first_word(str :&String) -> &str{ //Todo: &str is slice
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &str[..i];
        }
    }
    &str[..]
}