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
                Arg::new("action")
                    .short('c')
                    .long("command")
                    .value_parser(["new", "list", "del"])
                    .ignore_case(true)
                    .help("<new> for new password, <list> for password list, <del> to delete a password."))
            .get_matches();

    if let Some(answer) = matches.get_one::<String>("action") {
        if answer == "new" {
            new_password_saver();
        } else if answer == "list" {
            password_list();
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
    let connection = sqlite::open("my_keys.db").unwrap();

    let query = format!(
        "
        INSERT INTO passwords VALUES ('{url}', '{email}', '{new_password}');
    "
    );
    connection.execute(query).unwrap();
}

fn password_list() {
    let query = "SELECT website FROM passwords";
    let mut count = 0;

    let connection = sqlite::open("my_keys.db").unwrap();

    connection
        .iterate(query, |pairs| {
            for &(website, email) in pairs.iter() {
                count += 1;
                println!(
                    "{}, {}, {}",
                    count,
                    website.to_string(),
                    email.unwrap().to_string()
                )
            }
            true
        })
        .unwrap();

    let outcome = select_specific();
    password_database::lib::copy_to_clipboard(&outcome);
}

fn select_specific() -> String{
    println!("Which website do you need the password for?");
    let choice = input_capture();
    let mut string_to_copy = String::from("");
    let query = format!("SELECT password FROM passwords WHERE website = '{choice}'");
    let connection = sqlite::open("my_keys.db").unwrap();
    connection
        .iterate(query, |pairs| {
            for &(_column, password) in pairs.iter() {
                println!("The password is: , {}", password.unwrap().to_string());
                string_to_copy.push_str(password.unwrap());
            }
            true
        })
        .unwrap();
    string_to_copy

}
