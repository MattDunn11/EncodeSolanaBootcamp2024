// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints



fn main() {
    let optional_word = Some(String::from("rustlings"));

    // Changed the assignment to an if let statement
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // Changed the assignment to a while let statement
    while let Some(optional_integer) = optional_integers_vec.pop() {
        if let Some(integer) = optional_integer {
            println!("current value: {}", integer);
        }
    }
}

