fn main() {
    let performers = ["Rustful Five", "Rust in Peace", "Rustin Beiber"];
    let last = performers.into_iter().last().unwrap();
    println!("Last: {last}");

    let second = performers.into_iter().nth(1).unwrap();
    println!("Nth: {second}");

    let second_last = performers.into_iter().nth_back(1).unwrap();
    println!("second last: {second_last}");

    let target_index = performers
        .into_iter()
        .position(|element| element == "Rustin Beiber")
        .unwrap();
    println!("target index: {target_index}");
}
