// Reference and Borrowing
fn main() {
    let s1 = String::from("Sending...!");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    let mut name = String::from("Lucia");
    change(&mut name);

    // Multiple mutable references
    let mut algo = String::from("Hashmap");

    {
        let algo_a = &mut algo;
        algo_a.push_str(" (a)");
        println!("Algo a: {algo_a}");
    } // algo_a goes out of scope here, so we can make a new reference

    let algo_b = &mut algo;
    algo_b.push_str(" (b)");
    println!("Algo b: {algo_b}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(sub_name: &mut String) {
    sub_name.push_str(", Caminos");
    println!("The new subname is {sub_name}");
}