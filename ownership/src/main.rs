fn cloneheap(){
    let s1 = String::from("Running");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}\n\n");

    let (s2, len) = calculate_len(s1);
    println!("The length of {s2} is {len}!");
}

fn calculate_len(s: String) ->(String, usize){
    let length = s.len();
    (s, length)
}

fn main() {
    cloneheap();

    let s = String::from("Launch!"); // s comes into scope
    takes_ownership(s);              // s value is moved into to a function   

    let z = 10;                      // z comes into scope
    make_copy(z);                    // because i32 implements a copy trait
}                                    // z goes out of scope

fn takes_ownership(msg : String){
    println!("Message: {msg}");
}       //String goes out of scope, drop function is called and memory is freed

fn make_copy(a: i32){
    println!("Integer: {a}");
}
