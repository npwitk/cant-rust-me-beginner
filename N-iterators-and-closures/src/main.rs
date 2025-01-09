// .collect() -> แปลง .map ให้เป็น Vector type ตามเดิม
fn main() {
    let add = |a, b| a + b;
    let result = add(3, 4);
    println!("{}", result);

    let treasures = vec![100, 200, 300, 400];
    let doubled_treasures: Vec<i32> = treasures.iter().map(|x| x * 2).collect(); // |x| อันนี้คือ variable

    println!("{:?}", doubled_treasures);
}
