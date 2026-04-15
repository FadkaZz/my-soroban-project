#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// ================== STRUCT ==================
#[contracttype]
#[derive(Clone)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}

// ================== STORAGE KEY ==================
const NOTE_DATA: Symbol = symbol_short!("NOTE");

// ================== CONTRACT ==================
#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {

    // 🔍 GET ALL NOTES
    pub fn get_notes(env: Env) -> Vec<Note> {
        env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env))
    }

    // ➕ CREATE NOTE
    pub fn create_note(env: Env, title: String, content: String) -> String {
        let mut notes: Vec<Note> =
            env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        let note = Note {
            id: env.ledger().timestamp(), // ✅ FIX
            title,
            content,
        };

        notes.push_back(note);
        env.storage().instance().set(&NOTE_DATA, &notes);

        String::from_str(&env, "Note berhasil ditambahkan")
    }

    // 🔎 GET NOTE BY ID
    pub fn get_note_by_id(env: Env, id: u64) -> Vec<Note> {
    let notes: Vec<Note> =
        env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    let mut result = Vec::new(&env);

    for i in 0..notes.len() {
        if let Some(note) = notes.get(i) {
            if note.id == id {
                result.push_back(note);
            }
        }
    }
    result
}


    // ✏️ UPDATE NOTE
    pub fn update_note(env: Env, id: u64, new_title: String, new_content: String) -> String {
        let mut notes: Vec<Note> =
            env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..notes.len() {
            if let Some(mut note) = notes.get(i) {
                if note.id == id {
                    note.title = new_title;
                    note.content = new_content;

                    notes.set(i, note);
                    env.storage().instance().set(&NOTE_DATA, &notes);

                    return String::from_str(&env, "Note berhasil diupdate");
                }
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }

    // ❌ DELETE NOTE
    pub fn delete_note(env: Env, id: u64) -> String {
        let mut notes: Vec<Note> =
            env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

        for i in 0..notes.len() {
            if let Some(note) = notes.get(i) {
                if note.id == id {
                    notes.remove(i);
                    env.storage().instance().set(&NOTE_DATA, &notes);

                    return String::from_str(&env, "Note berhasil dihapus");
                }
            }
        }

        String::from_str(&env, "Note tidak ditemukan")
    }
}