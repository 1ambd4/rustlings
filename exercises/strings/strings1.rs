// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

// &str is not String, and can not convert &str to String
// So, the one solution is change the return type to &str
// and the other is change the return data's type to String.
// fn current_favorite_color() -> &'static str {
fn current_favorite_color() -> String {
    String::from("blue")
}
