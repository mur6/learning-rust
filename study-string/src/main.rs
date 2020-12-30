fn set_hello_world(str: &mut String) {
    str.push_str("Hello World.");
}

fn main() {
    let mut s = "".to_string();
    set_hello_world(&mut s);
    println!("{}", s);
}
