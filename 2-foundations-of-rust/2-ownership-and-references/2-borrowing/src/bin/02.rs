//! You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> Option<char> {
    match data.chars().last(){
        Some(v) => Some(v),
        None => None,
    }
    // data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
