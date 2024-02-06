pub fn basics() {
    let x = 5;
    let y = 10;

    let z = 2.5; // float

    // x + y
    let sum = x as f64 - z;

    // boolean
    let mut is_tall = true;
    is_tall = false;

    // Chars
    let mut my_char = 'A';
    my_char = 'b';

    // String
    let mut first_name = "Bob";
    first_name = "Peter";

    // println!("Name: {}", first_name);

    // Arrays sames datatype / list
    let hero_names = ["Bruce", "Diana", "Clerk"];
    // println!("Heroes: {:?}", hero_names);

    let heros_ages = [42, 34, 55, 65, 90];
    let first_ages = &heros_ages[1..=3];

    println!("Ages: {:?}", first_ages);
}
