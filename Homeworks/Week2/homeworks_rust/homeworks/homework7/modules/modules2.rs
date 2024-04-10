// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)



mod delicious_snacks {
    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

// Correct use statements using `as` for renaming
use delicious_snacks::fruits::PEAR as fruit;
use delicious_snacks::veggies::CUCUMBER as veggie;

fn main() {
    println!(
        "favorite snacks: {} and {}",
        fruit,
        veggie
    );
}


