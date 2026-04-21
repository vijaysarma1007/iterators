fn main() {
    let fifty_numbers = 1..=50;

    for number in fifty_numbers.take(15).step_by(2) {
        println!("{number}/");
    }
}
