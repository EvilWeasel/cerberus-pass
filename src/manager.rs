use cerberus_pass::utils::{combine_prompt, prompt_user};

#[derive(Debug)]
pub struct PasswordEntry {
    title: String,
    login: String,
    password: String,
    website: Option<String>,
    note: Option<String>,
}
pub struct PasswordManager {
    vault: Vec<PasswordEntry>,
}
impl PasswordManager {
    pub fn new() -> Self {
        Self { vault: Vec::new() }
    }
    pub fn add_entry(&mut self) {
        let title = prompt_user(combine_prompt("title"));
        let login = prompt_user(combine_prompt("login"));
        let password = prompt_user(combine_prompt("password"));
        let website = prompt_user(combine_prompt("website"));
        let note = prompt_user(combine_prompt("note"));
        self.vault.push(PasswordEntry {
            title,
            login,
            password,
            website: Some(website),
            note: Some(note),
        });
    }
    pub fn get_entry(&self, title: &str) -> Option<&PasswordEntry> {
        self.vault.iter().find(|entry| entry.title == title)
    }
    pub fn get_list(&self) -> &Vec<PasswordEntry> {
        &self.vault
    }
}
