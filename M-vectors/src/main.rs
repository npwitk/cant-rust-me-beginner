fn main() {
    let mut items = vec!["Gold", "Silver", "Ruby Gem"];

    items.remove(1);
    items.push("Diamond");

    println!("Items: {:?}", items);
    println!("Items length: {}", items.len());
    println!("Items capacity: {}", items.capacity());
    // เวลาจะเพิ่ม Rust จะเพิ่มที่ละ 2 เท่า
}
