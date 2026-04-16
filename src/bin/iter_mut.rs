fn main() {
    let mut flavors = [
        String::from("Chocolate"),
        String::from("vanilla"),
        String::from("Strawberry"),
    ];

    let iterator = flavors.iter_mut();

    for flavour in iterator {
        flavour.push_str(" Ice Cream");
    }

    println!("{flavors:?}");


    let mut school_grades = [85,90,92, 75];
    
    for grade in &mut school_grades {
        *grade -=2;
    }

    println!("{school_grades:?}");
}
