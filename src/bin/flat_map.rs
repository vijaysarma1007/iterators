fn main() {
    let attendees = [
        "Bob, marry, Kevin",
        "Mike, Robbie, Matt, Austin",
        "Piers, Liam",
    ];

    let attendees: Vec<&str> = attendees
        .iter()
        .flat_map(|group| group.split(", "))
        .collect();
    println!("{attendees:?}");
}
