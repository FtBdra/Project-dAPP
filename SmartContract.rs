#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, vec, Env, String, Symbol, Vec};

// Struktur data Mahasiswa Data Sains
#[contracttype]
#[derive(Clone, Debug)]
pub struct Student {
    pub id: u64,
    pub name: String,
    pub nim: String,
    pub expertise: String, // Contoh: Machine Learning, NLP, Big Data
}

const STUDENT_DATA: Symbol = symbol_short!("STU_DATA");

#[contract]
pub struct StudentRegistryContract;

#[contractimpl]
impl StudentRegistryContract {
    // Mendapatkan semua data mahasiswa
    pub fn get_students(env: Env) -> Vec<Student> {
        env.storage()
            .instance()
            .get(&STUDENT_DATA)
            .unwrap_or(vec![&env])
    }

    // Menambahkan mahasiswa baru
    pub fn register_student(env: Env, name: String, nim: String, expertise: String) -> u64 {
        let mut students: Vec<Student> = Self::get_students(env.clone());
        
        let id = students.len() as u64 + 1;
        let new_student = Student {
            id,
            name,
            nim,
            expertise,
        };

        students.push_back(new_student);
        env.storage().instance().set(&STUDENT_DATA, &students);
        
        id
    }

    // Menghapus mahasiswa berdasarkan ID
    pub fn delete_student(env: Env, id: u64) -> String {
        let students: Vec<Student> = Self::get_students(env.clone());
        let mut new_students: Vec<Student> = vec![&env];
        let mut found = false;

        for item in students.iter() {
            if item.id != id {
                new_students.push_back(item);
            } else {
                found = true;
            }
        }

        if found {
            env.storage().instance().set(&STUDENT_DATA, &new_students);
            String::from_str(&env, "Success: Student removed")
        } else {
            String::from_str(&env, "Error: Student ID not found")
        }
    }
}
