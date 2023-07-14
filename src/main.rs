use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::io;

mod search;
use search::{get_file_data, FileData};

fn main() -> io::Result<()> {
    let dir_path = read_user_input("Enter path:");
    let query = read_user_input("Search:");

    let get_file_data = get_file_data(&dir_path);
    let files = get_file_data?;

    let matcher = SkimMatcherV2::default();

    let mut filtered_files: Vec<FileData> = files
        .into_iter()
        .filter(|file| matcher.fuzzy_match(&file.name, &query).is_some())
        .collect();

    let sorted_files = &mut filtered_files;

    sorted_files.sort_by(|a, b| b.size.partial_cmp(&a.size).unwrap());

    for file in &mut *sorted_files {
        println!("{}", file.name);
        println!("{}", file.path);
        println!("{:.2} KB\n", file.size);
    }

    println!("Total of {} files.", sorted_files.len());

    Ok(())
}

fn read_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);

    if result.is_ok() {
        input.trim().replace('\\', "/").to_string()
    } else {
        println!("No query.");
        String::new()
    }
}

// fn get_directory_size(dir_path: &Path) -> io::Result<f64> {
//     let mut total_size: f64 = 0.0;
//     let entries = fs::read_dir(dir_path)?;

//     for entry in entries {
//         let entry = entry?;
//         let file_type = entry.file_type()?;
//         let path = entry.path();

//         if file_type.is_file() {
//             let size_in_kb: f64 = get_file_size(&path)?;
//             total_size += size_in_kb;
//         } else if file_type.is_dir() {
//             let size_in_kb: f64 = get_directory_size(&path)?;
//             total_size += size_in_kb;
//         }
//     }

//     Ok(total_size)
// }
