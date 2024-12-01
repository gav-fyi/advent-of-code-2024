use utils::read_lines_from_file;
use utils::split_lines_into_vectors;
use utils::sum_of_vector;


fn main() {

    let mut scores: Vec<i32> = Vec::new();

    match read_lines_from_file("data.txt") {
        Ok(lines) => {
            match split_lines_into_vectors(lines) {
                Ok((vec_a, vec_b)) => {
                    for &a in &vec_a {
                        let count = vec_b.iter().filter(|&&b| b == a).count() as i32;
                        scores.push(a * count);
                    }
                }
                Err(e) => eprintln!("Error splitting lines: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("\nAnswer: {:?}", sum_of_vector(scores));

}
