fn main() {
    println!("Test for onwer and deep/shallow copy.");
    let s1 = String::from("Hello copy.");
    let s2 = s1; //EP: move, shallow copy
    println!("{}", s2);
    
}
