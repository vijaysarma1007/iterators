fn main() {
    let teas = [
        String::from("Hot Earl Grey"),
        String::from("Iced Green"),
        String::from("Hot Matcha"),
    ];

    let more_teas: Vec<String> = teas.iter().cloned().collect();

    let alt_more_teas: Vec<String> = teas.clone().into_iter().collect();
}
