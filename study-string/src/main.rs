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
    // 1.
    let s = "abc";
    println!("{}", s);
    // 2. concat
    let hello = "hello".to_string();
    let world = "world";
    let s2 = hello + world;
    println!("{}", s2);
    // 3. 比較
    if s == "abc" {
        println!("同じ!");
    }
    // 4. 文字列フォーマット
    // 5. join
    //let vs = vec!["Foo".to_string(),"bar".to_string(), "baz".to_string()];
    let vs = vec!["foo", "bar", "baz"];
    let s3 = vs.join("-");
    println!("{}", s3);
}
