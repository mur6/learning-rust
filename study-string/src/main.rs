fn adder(x: i32, y: i32) -> i32 {
    x + y
}

fn set_hello_world(str: &mut String, num: i32) {
    let ans = adder(5, num);
    let s = format!("Hello World: ans={}", ans);
    str.push_str(&s);
}

fn main() {
    let mut s = "".to_string();
    set_hello_world(&mut s, 3);
    println!("{}", s);
}
