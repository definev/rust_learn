
/// Patterm matching has a concept of refutability:
/// - If patterm matching with enum it has some type of enum so rust-compiler refuse direct patterm match.
/// You must use `if let` for check type of enum
/// 
/// Some patterm matching:
/// - Match exact variable name
/// - Ignore variable with `_`
/// - Get specific variable and ignore all use `..` syntax. Ex: (.., variable)
/// - Use `|` syntax for declare multiple value matching. Ex: 1 | 2 | 3 => println!("Less than 3"),
/// - Use condition for filtering value in pattermatching with `if` keyword. Ex: 1 | 2 | 3 if true => println!("Less than 3"),
fn main() {

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
