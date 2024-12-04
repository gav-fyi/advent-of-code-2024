use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub fn read_lines_from_file(file_path: &str) -> Result<Vec<String>, io::Error> {
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    reader.lines().collect()
}


pub fn read_string_from_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn split_ints(input: &str) -> Result<[i32; 2], &'static str> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Input must contain exactly two integers separated by whitespace");
    }
    let first: i32 = parts[0].parse::<i32>().map_err(|_| "Failed to parse the first integer")?;
    let second: i32 = parts[1].parse::<i32>().map_err(|_| "Failed to parse the second integer")?;
    Ok([first, second])
}

pub fn split_lines_into_vectors(lines: &Vec<String>) -> Result<(Vec<i32>, Vec<i32>), &'static str> {
    let mut vec_a = Vec::new();
    let mut vec_b = Vec::new();

    for line in lines {
        let [a, b] = split_ints(&line)?;
        vec_a.push(a);
        vec_b.push(b);
    }

    Ok((vec_a, vec_b))
}

pub fn sort_vectors(vec_a: &mut Vec<i32>, vec_b: &mut Vec<i32>) {
    vec_a.sort();
    vec_b.sort();
}

pub fn calculate_diff(a: &i32, b: &i32) -> i32 {
    (a-b).abs()
}

pub fn sum_of_vector(numbers: &Vec<i32>) -> i32 {
    numbers.iter().sum()
}
