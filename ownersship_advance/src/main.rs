use std::io::stdout;

fn main() {
    let s_taken = String::from("Hello world");
    take_ownership(s_taken);
    // TODO: this cause error because the ownership of String s_taken has been taken in function take_ownership
    //println!("s_taken: {}", s_taken);

    let s_giveback : String = String :: from("Hi Rust 💕🥰");
    //TODO: cannot s_giveback = take_and_giveback_ownership(s_giveback); cuz s_giveback is immutable
    let s_back =  take_and_giveback_ownership(s_giveback);
    println!("s_giveback: {}", s_back);

    let num :i32 = 5;
    makes_copy(num);
    println!("num: {}", num);
}

fn take_ownership(str: String){
    println!("{}", str);
}

fn take_and_giveback_ownership(str: String) -> String{
    println!("{}", str);
    str //return
}

fn makes_copy(num: i32){
    println!("{}", num);
}

fn calculate_string_length(str: String) -> (String, usize){
    let len = str.len();
    return (str, len); //return
}
// slice, see next
fn first_word(str: &String) -> usize{
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if (item == b' ') {
            return i;
        }
    }
    str.len()
}
