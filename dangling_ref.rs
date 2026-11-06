fn main(){
    /* let ref_to_nothing = dangle(); */
    let ref_to_nothing = no_dangle();

}

/* fn dangle() -> &String{
    let s = String::from("Kenny!");
    &s
} */

fn no_dangle() -> String{
    let s = String::from("kenny");
    s
}