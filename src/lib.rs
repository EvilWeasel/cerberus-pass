pub mod utils {
    use std::io::{self, Write};

    /// Prompt the user with an inline prompt like `question: {userInput}`.
    /// Returns the `userInput` as an **owned and trimmed `String`**.
    /// Crashes the program, when `io::stdout` or `io::stdin` fails for any reason. See `io::stdout.flush` and `io::stdin.read_line` documentation for details.
    pub fn prompt_user(prompt: String) -> String {
        let mut buf = String::new();
        print!("{}: ", prompt);
        io::stdout()
            .flush()
            .expect("EOF reached... probably, idk 😩");
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        buf.trim().to_string()
    }

    pub fn combine_prompt(name: &str) -> String {
        format!("Enter the {} for the entry", name)
    }
}
