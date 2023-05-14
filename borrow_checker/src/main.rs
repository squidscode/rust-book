fn main() {
    let mut s : String = String::from("Hello there!");
    let a : &mut String = &mut s;
    // let b : &String = &s;

    drop(a);
    println!("The string is: {}", a);
}

fn drop(s : &mut String) {
    // does nothing
}