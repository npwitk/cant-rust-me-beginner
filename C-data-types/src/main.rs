fn main() {
    let x = 1;
    let y = 12.0;

    let z = x + y as i32;

    let msg: String = String::from("Hello, world!");
    let msg2: String = "Hello, world!".to_string();
    let msg3: &str = "Hello, world!";
    let msg4: String = format!("Position: {}, {}", x, y);

    println!("{}", msg4);
}
