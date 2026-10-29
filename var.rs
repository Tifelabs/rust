fn main() {
    const VERSION: u32 = 7;

    let mut x = 5;
    let m: i8 = 67;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("Current version: {VERSION}");

    {
        let x = x * 5;
        println!("The value of x in the inner scope is: {x}\n");
    }

    println!("Signed: {m}\n\n");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (i, j, k) = tup;                       

    println!("The value of i: {i}");
    println!("The value of j: {j}");
    println!("The value of k: {k}");

    let arr: [i8; 4] = [-3, -4, -6, -17];
    println!("The value of the 3rd index: {}", arr[2]);
}