use std::ptr::addr_of;


fn main() {
    println!("Ownership");
    unsafe{
        create_vector_and_move_owership();
        create_vector_and_clone();
    }
}

fn create_vector_and_move_owership(){
    let v1 = vec![1, 2, 3, 4,];
    println!("v1[1] = {} : in addr {:p}", &v1[1], addr_of!(v1[1]));
    let v2 = v1;
    println!("v2[1] = {} : in addr {:p}", &v2[1], addr_of!(v2[1]));
    print!("If we make this unsafe and try to print v1[1]'s addr:");
    println!("\tNo, It'll still error.");
    // unsafe{
    //     println!("v1[1] = {} : in addr {:?}", *v1.get_unchecked(1), v1.as_ptr().add(1));
    // }
    println!();
    //println!("v1[1] = {} : in addr {:p}", &v1[1], addr_of!(v1[1]));

}

unsafe fn create_vector_and_clone(){
    let v1 = vec![1, 2, 3, 4,];
    println!("v1[1] = {} : in addr {:p}", &v1[1], addr_of!(v1[1]));
    let v2 = v1.clone();
    println!("If we clone and try to print v1[1]'s addr:");
    //println!("No, It'll still error.")
    println!("v1[1] = {} : in addr {:p}", &v1[1], addr_of!(v1[1]));
    println!("v2[1] = {} : in addr {:p}", &v2[1], addr_of!(v2[1]));
    println!("It works!")
    
}