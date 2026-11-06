fn main(){
    let word = String::from("Lunch Deck");
    let f_word = &word[0..5];
    let s_word = &word[6..10];

    let len = word.len();
    let slice = &word[0..len];
    
    let a = [45, 34, 56, 32, 40];
    let a_slice = &a[1..3];
    assert_eq!(a_slice, &[2, 3]);
    println!("Full Word: {word}\nFirst Word: {f_word}\nSecond Word: {s_word}\n");
    println!("Word Length: {len}\n");
    println!("First Slice: {slice}\n");
    println!("Other Slice: {a_slice}");   
}