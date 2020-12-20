// code from here: https://www.ameyalokare.com/rust/2017/10/23/rust-options.html
fn get_alias(name: &str) -> Option<&str> {
    match name {
        "Bob" => Some("The Builder"),
        "Elvis" => Some("The King"),
        _ => None,
    }
}

fn main() {
    struct FullName {
        first: String,
        middle: Option<String>,
        last: String,
    };

    let alice = FullName {
        first: String::from("Alice"),
        middle: Some(String::from("Bob")), // Alice has a middle name
        last: String::from("Smith")
    };
    
    let jon = FullName {
        first: String::from("Jon"),
        middle: None, // Jon has no middle name
        last: String::from("Snow")
    };

    println!("Alice's middle name is {}",
            match alice.middle {
                None => "No middle name!",
                Some(ref x) => x, // x is now a string slice
            }
    );

    println!("Jon's middle name is {}",
    jon.middle.unwrap_or("No middle name!".to_owned()));

    println!(
        "Alice's full name is {} {} {}",
        alice.first,
        alice.middle.as_ref().map_or("", |m| &m[0..1]), // as_ref() converts Option<String> to Option<&String>
        alice.last
    );

    let optional_nickname = alice.middle.as_ref().and_then(|m| get_alias(&m));

    println!("Alice's middle name's nickname is {}",
    optional_nickname.unwrap_or("(none found)"));
}
