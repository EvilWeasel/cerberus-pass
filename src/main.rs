// todo: definetly need to remove this
#![allow(dead_code)]
use cerberus_pass::utils::{combine_prompt, prompt_user};
use manager::PasswordManager;
mod manager;
fn main() {
    let mut manager = PasswordManager::new();
    println!("Welcome to Cerberus-Pass");
    loop {
        let prompt = "
        1. List all entries in vault
        2. Create new entry
        3. Get an entry
        4. Update an entry
        5. Delete an entry
        "
        .to_string();
        let choice = prompt_user(prompt);

        let choice = match choice.parse::<i8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                manager
                    .get_list()
                    .iter()
                    .for_each(|entry| println!("{:?}", entry));
            }
            2 => {
                let title = prompt_user(combine_prompt("title"));
                let login = prompt_user(combine_prompt("login"));
                let password = prompt_user(combine_prompt("password"));
                let website = prompt_user(combine_prompt("website"));
                let note = prompt_user(combine_prompt("note"));
                manager.add_entry(&title, &login, Some(&website), Some(&note), &password);
            }
            3 => {
                // input title to get io
                let title = prompt_user(combine_prompt("title"));
                let maybe_entry = manager.get_entry(&title); // get maybe_entry for title
                match maybe_entry {
                    Some(entry) => {
                        println!("{:?}", entry);
                        // decrypt password in entry with master-password

                        //println!(maybe_entry)
                    }
                    None => {
                        println!(
                            "No entry with {} was found in vault. Maybe you spelled it wrong?",
                            title
                        );
                    }
                }
            }
            4 => {
                println!("Update functionality not implemented yet.");
            }
            5 => {
                println!("Delete functionality not implemented yet.");
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
