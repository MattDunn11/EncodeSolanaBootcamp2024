// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references



fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("Last character: {}", last_char);

   let uppercase_data = string_uppercase(&data);
   println!("Uppercase string: {}", uppercase_data);
}

// Should not take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &String) -> String {
    let uppercase_data = data.to_uppercase();
    println!("{}", uppercase_data);

    uppercase_data
}
