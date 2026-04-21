fn main() {
    let earnings = [4, 7, 9, 13];
    let sum = earnings
        .into_iter()
        .reduce(|total, current| total + current);
    println!("{sum:?}");

    let address_portions = [String::from("123 Elm street"), String::from("Suburbia")];

    let res = address_portions.join(", ");
    println!("{res}");
}
