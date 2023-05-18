use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};

fn main() {
    let file_path_to_read = "test_file";
    read_and_filter_lines(file_path_to_read, "a");
    read_using_file(file_path_to_read);

    let file_path_to_write = "test_file_to_write";
    write_using_fs(file_path_to_write);

    let lines_of_words = read_words_from_file(file_path_to_read).unwrap();

    let file_path_to_write = "test_file_to_write_with_words";
    write_words_to_file(file_path_to_write, &lines_of_words.join(&"\n".to_string())).unwrap();

    let words = lines_of_words.into_iter().flatten().collect::<Vec<String>>();
    let count = word_count(&words);
    println!("{:?}", count);

    let dict = HashMap::from([
        ("line".to_string(), "L".to_string()) ,
        ("file".to_string(), "FILE".to_string()),
    ]);
    let words = replace_words(words, dict);
    println!("{:?}", words);
}

fn write_words_to_file(file_path: &str, words: &Vec<String>) -> Result<(), io::Error>{
    fs::write(file_path, words.join(" "))?;
    Ok(())
}

fn write_using_fs(file_path: &str) {
    let contents = "This is my new file! Congrats!!!";
    fs::write(file_path, contents).unwrap();
}

fn read_using_fs(file_path: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents.lines().map(|line| line.to_string()).collect())
}

fn read_file(file_path: &str) -> Vec<String> {
    read_using_fs(file_path).expect(&format!("Unable to read file {}", file_path))
}

fn read_and_filter_lines(file_path: &str, wanted_string: &str) {
    let contents = read_file(file_path);

    for line in contents {
        if line.contains(wanted_string) {
            println!("{line}");
        }
    }
}

fn read_using_file(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}

fn read_words_from_file(file_path: &str) -> Result<Vec<Vec<String>>, io::Error>{
    Ok(read_using_fs(file_path)?
        .iter()
        .map(|line| get_words(line))
        .collect())
}

fn get_words(line: &str) -> Vec<String> {
    line.split_whitespace().map(|word| word.to_string()).collect()
}

fn word_count(words: &Vec<String>) -> HashMap<String, i32> {
    let mut hash_map = HashMap::new();
    for word in words {
        hash_map
            .entry(word.to_lowercase())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    hash_map
}

fn replace_words(mut words: Vec<String>, dict: HashMap<String, String>) -> Vec<String> {
    for word in words.iter_mut() {
        if let Some(new_word) = dict.get(word) {
            *word = new_word.to_string();
        }
    }
    words
}
