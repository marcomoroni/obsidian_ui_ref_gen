//! This is a script for my own Obsidian vault.
//!
//! For each of my UI references files, it generates a corresponding Obsidian note, so that each
//! file can be tagged, linked, etc.
//!
//! Usage:
//!
//! ```
//! obsidian_ui_ref_gen.exe <folder_with_files> <output_folder>
//! ```

use rand::{rngs::ThreadRng, Rng};
use std::{env, fs, path::PathBuf};

const UNTITLED_ID_DIGITS: usize = 12;

fn main() {
    let args: Vec<String> = env::args().collect();

    let folder_to_query = &args[1];
    let folder_out = &args[2];

    let mut rng = rand::thread_rng();

    generate_ui_ref_notes(folder_to_query, folder_out, &mut rng);
}

fn generate_ui_ref_notes(folder_to_query: &String, folder_out: &String, rng: &mut ThreadRng) {
    if let Ok(entries) = fs::read_dir(folder_to_query) {
        for entry in entries {
            if let Ok(entry) = entry {
                let generated_file_id = random_note_id(rng);
                let generated_file_name = format!("UI reference {}.md", generated_file_id);
                let generated_file_content = format!(
                    "[[UI reference ◆]]

image:: ![[{}]]",
                    entry.file_name().into_string().unwrap()
                );
                let generated_file_path = PathBuf::from(folder_out).join(&generated_file_name);
                std::fs::write(generated_file_path, generated_file_content).unwrap();

                println!(
                    "Generated: {:?} -> \"{}\"",
                    entry.file_name(),
                    generated_file_name
                )
            }
        }
    }
}

/// A random string of fixed lenght of digits.
fn random_note_id(rng: &mut ThreadRng) -> String {
    let mut id = "".to_string();
    for _ in 0..UNTITLED_ID_DIGITS {
        id = format!("{}{}", id, rng.gen_range(0..10));
    }
    id
}
