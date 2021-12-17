enum ConnectionState {
    Waiting,
    Connected(String),
    Error(NetworkError),
}

struct NetworkError {
    code: u16,
    message: String,
}

fn main() {
    let waiting = ConnectionState::Waiting;
    let connected = ConnectionState::Connected(String::from("We have Connected!"));
    let error = ConnectionState::Error(NetworkError {
        code: 404,
        message: String::from("Page not found!"),
    });


    display_result(&waiting);
    display_result(&connected);
    display_result(&error);
}

fn display_result(state: &ConnectionState) {
    // The matching patterm is so powerful
    // We can use this for enum and with value stick with that, we will excute a specific piece of code
    match state {
        ConnectionState::Waiting => {
            println!("Wait a sec ...\n");
        }
        ConnectionState::Connected(result) => {
            println!("Data: {}\n", result);
        }
        ConnectionState::Error(err) => {
            println!("Code: {}\nMessage: {}\n", err.code, err.message);
        }
    }
}
