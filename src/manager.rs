use cerberus_pass::utils::{combine_prompt, prompt_user};

/// Represents an entry that only includes metadata (no password).
/// Useful for listing entries to the user without exposing passwords.
#[derive(Debug)]
pub struct PasswordEntryPublic {
    pub title: String,
    pub login: String,
    pub website: Option<String>,
    pub note: Option<String>,
}

/// Represents a fully encrypted entry. The password is stored in encrypted form.
#[derive(Debug)]
struct PasswordEntryEncrypted {
    title: String,
    login: String,
    website: Option<String>,
    note: Option<String>,

    // TODO: store the encrypted password here. For now, just store it as plain String to illustrate.
    encrypted_password: String,
    // TODO: store encryption-related metadata here (like nonce, salt for encryption, etc.).
}

/*
    int x = 10;
    int y = x;
    x = 22;
    y == 10

    Person p1 = new Person() { Name = "John" };
    Person p2 = p1;
    p1.Name = "Jürgen";
    p2.Name == "Jürgen"
*/

/// Core manager that contains encrypted entries. Calls that need to decrypt
/// should ask for a key or the master password.
pub struct PasswordManager {
    vault: Vec<PasswordEntryEncrypted>,
}

impl PasswordManager {
    /// Creates a new, empty manager.
    /// This will not handle any master-password logic directly (that is up to API consumers).
    pub fn new() -> Self {
        Self { vault: Vec::new() }
    }

    /// Adds a new entry to the vault.
    /// Collects plaintext data, then encrypts the password (TODO: real encryption).
    pub fn add_entry(
        &mut self,
        title: &str,
        login: &str,
        website: Option<&str>,
        note: Option<&str>,
        plaintext_password: &str,
    ) {
        // TODO: This would normally prompt the user for input, handle encryption here.
        // For now, we just store a placeholder.
        let title = title.to_string();
        let login = login.to_string();
        let website = website.map(|s| s.to_string());
        let note = note.map(|s| s.to_string());

        // TODO: Actually encrypt `plaintext_password`.
        let encrypted_password = plaintext_password.to_string();

        self.vault.push(PasswordEntryEncrypted {
            title,
            login,
            website,
            note,
            encrypted_password,
        });
    }

    /// Returns a list of "public" entries, not including passwords.
    /// Useful for display or listing purposes.
    pub fn get_list(&self) -> Vec<PasswordEntryPublic> {
        self.vault
            .iter()
            .map(|entry| PasswordEntryPublic {
                title: entry.title.clone(),
                login: entry.login.clone(),
                website: entry.website.clone(),
                note: entry.note.clone(),
            })
            .collect()
    }

    /// Returns the a single entry by title
    pub fn get_entry(
        &self,
        title: &str, /*, TODO: add key or master pass here */
    ) -> Option<PasswordEntryPublic> {
        // Find matching entry and cast to public type.
        self.vault
            .iter()
            .find(|entry| entry.title == title)
            .map(|entry| PasswordEntryPublic {
                title: entry.title.clone(),
                login: entry.login.clone(),
                website: entry.website.clone(),
                note: entry.note.clone(),
            })
    }

    /// Updates an existing entry without altering the password.
    /// If you need to update the password, call a separate method that does re-encryption.
    /// Returns an option with the updated entry as public data.
    pub fn update_entry(
        &mut self,
        title: &str,
        new_title: &str,
        new_login: &str,
        new_website: &str,
        new_note: &str,
    ) -> Option<PasswordEntryPublic> {
        // TODO: locate the entry, update metadata (title, login, website, note).

        // Find the entry by title and update its metadata.
        if let Some(entry) = self.vault.iter_mut().find(|entry| entry.title == title) {
            entry.title = new_title.to_string();
            entry.login = new_login.to_string();
            entry.website = Some(new_website.to_string());
            entry.note = Some(new_note.to_string());
            // Return the updated entry as public data.
            Some(PasswordEntryPublic {
                title: entry.title.clone(),
                login: entry.login.clone(),
                website: entry.website.clone(),
                note: entry.note.clone(),
            })
        } else {
            // Return None if the entry was not found.
            None
        }
    }

    /// Deletes an entry from the vault by title.
    pub fn delete_entry(&mut self, title: &str) {
        self.vault.retain(|entry| entry.title != title);
    }
}
