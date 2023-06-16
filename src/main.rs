use clap::{Arg, Command};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlite;
use std::io;

fn main() {
    let matches = Command::new("My Password Manager")
            .author("Maphikza, Mapz@me.com")
            .version("0.0.1")
            .about("This is an app for generating safe passwords and storing them in your private database")
            .arg(
                Arg::new("create")
                    .short('c')
                    .help("for creating new password."))
            .get_matches();

    if let Some(answer) = matches.get_one::<String>("create") {
        if answer == "1" {
            new_password_saver();
        } else {
            println!("missing argument.")
        }
    }
}

fn input_capture() -> String {
    let mut item = String::new();

    io::stdin()
        .read_line(&mut item)
        .expect("couldn't read line.");

    let item: String = item
        .trim()
        .parse()
        .expect("This is supposed to be a string.");

    return item;
}

fn password_generator() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    return rand_string;
}

fn new_password_saver() {
    println!("What is the web address");
    let url = input_capture();
    println!("What is the username or password?");
    let email = input_capture();
    let new_password = password_generator();
    let connection =
        sqlite::open("my_keys.db").unwrap();

    let query = format!(
        "
        INSERT INTO passwords VALUES ('{url}', '{email}', '{new_password}');
    "
    );
    connection.execute(query).unwrap();
}
