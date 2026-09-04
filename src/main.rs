use std::fs;
use std::time::SystemTime;

fn load_lines(path: &str) -> Vec<String> { 
    // load file content into String vector:
    let content: Vec<String> = fs::read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .map(|line: &str| line.to_string())
        .collect();
    //println!("content: {:#?}", content);
    return content;
}

fn unique_chars(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars.dedup();
    chars
}

fn randomize(strings: &[String]) -> Vec<String> {
    let mut v = strings.to_vec();
    let t = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut seed = t; // or from time/entropy

    for i in (1..v.len()).rev() {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let j = (seed % (i as u64 + 1)) as usize;
        v.swap(i, j);
    }
    v
}

fn join_strings(strings: &[String], delimiter: &str) -> String {
    let str = strings.join(delimiter);
    return str;
}   

fn main() {
    let file: &str = "names.txt"; 
    let file2: String = String::from("names.txt");
    println!("Parsing '{file}' or {file2}");
    
    let data: Vec<String> = load_lines(&file2);
    
    let data_randomized: Vec<String> = randomize(&data);
    let uchars = unique_chars(&join_strings(&data, ""));
    let bos = uchars.len(); // beginning of sequence
    let vocab_size = bos+1;
    let unique_chars_string: String = uchars.iter().copied().collect();
    
    println!("Before: '{}', '{}'", data[0], data[1]);
    println!("After: '{}', '{}'", data_randomized[0], data_randomized[1]);
    println!("Number of lines: {} and unique chars {}", data.len(), unique_chars_string);

    println!("Done...");
}