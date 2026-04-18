fn main() {
    let applicants = vec![
        "Rob", "Bob", "Cob", "Alex", "Piers", "John", "Darnil", "Dan",
    ];

    let winners = applicants
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 3 == 0)
        .map(|(_, applicant)| applicant)
        .collect::<Vec<&str>>();

    println!("{winners:?}");
}
