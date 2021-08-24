use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Workout {
    rounds: usize,
    repetitions: u8,
    categories: Vec<Category>,
}

#[derive(Serialize, Deserialize)]
struct Category {
    name: String,
    exercises: Vec<String>
}

fn main() {
    print_intro();
    wait_for_user();

    let filepath = get_filepath();
    let file_contents = fs::read_to_string(filepath)
        .expect("Something went wrong when reading the file");

    let mut workout: Workout = serde_json::from_str(&file_contents).unwrap();

    for r in 0..workout.rounds {
        println!("Round {}", r+1);
        for c in 0..workout.categories.len() {
            let random_index = random::<usize>() % workout.categories[c].exercises.len();
            println!("{}", workout.categories[c].exercises[random_index]);
            workout.categories[c].exercises.remove(random_index);
        }
        println!("Repeat {} times", workout.repetitions);
        println!("");
    }
}

fn get_filepath() -> PathBuf {
    let mut path_buf = env::current_dir().unwrap();
    path_buf.push("src");
    path_buf.push("config");
    path_buf.push("exercises");
    path_buf.set_extension("json");
    return path_buf;
}

fn print_intro() {
    println!("Welcome to your workout of the day");
    println!("Once you have configured your exercises,");
    println!("click Enter to generate your workout.");
    println!("");
}

#[allow(unused_must_use)]
fn wait_for_user() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
}