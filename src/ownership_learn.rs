pub fn scoping_ownership() {
    {
        let my_str = "hello world"; //comes into scope

        println!("Inside {}", my_str);
    } //drop();
      // goes out scope

    // println!("{}", my_str); // wont't work
}

pub fn string_ownership() {
    let mut hero = String::new();

    hero.push_str("Hello ");

    println!("{}", hero);

    {
        let greet = String::from("Hello World");
    }
}

pub fn ownership_functions() {
    let hero = String::from("Clerk");

    getfull_name(&hero);

    get_work(&hero);

    println!("Outside his name is: {}", hero);
}

fn getfull_name(name: &String) {
    // comes into scope
    println!("The hero's name is: {}", name); // used
} //

fn get_work(name: &String) {
    println!("He works at: {}", name);
}

pub fn mutate_hero() {
    let mut hero_name = String::from("Diana");

    get_full_diana(&mut hero_name);
}

fn get_full_diana(name: &mut String) {
    name.push_str(" White");
    println!("Hero's full name is: {}", name);
}

pub fn ownership_integers() {
    let x = 24;

    // scope
    takes_ownership(&x);

    println!("x = {}", x);
}

fn takes_ownership(num: &i32) {
    let y = 10;
}
