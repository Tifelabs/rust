fn field_length(value: i32, label: char){
    println!("The field length is {value}{label} meters")
}
fn another_function(x: i32){
    println!("The value of x is: {x}");
}

fn five() ->i32{
    5
}

fn plus_one(y: i32) -> i32{
    y + 1
}


fn main() {
   another_function(67);
   field_length(100, 'm');

   let x = five();
   println!("The value of x is: {x}\n");

   let y = plus_one(9);
   println!("The value of y is: {y}\n");
}

