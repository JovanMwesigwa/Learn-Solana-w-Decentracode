pub fn my_first_func() {
    // greet_func("Jovan".to_string());

    let first_name = "Bruce".to_string();
    let last_name = "Wayne".to_string();

    let batman = get_fullname(first_name, last_name);
    println!("Batman name is {}", batman);
}

fn get_fullname(first: String, last: String) -> String {
    let full_name = format!("{0} {1}", first, last);
    full_name
}

fn greet_func(name: String) {
    println!("Good morning {}", name);
}
