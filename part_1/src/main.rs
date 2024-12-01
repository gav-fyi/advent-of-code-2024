use utils::read_lines_from_file;
use utils::split_lines_into_vectors;
use utils::sort_vectors;
use utils::calculate_diff;
use utils::sum_of_vector;


fn main() {

    let mut differences: Vec<i32> = Vec::new();

    match read_lines_from_file("data.txt") {
        Ok(lines) => {
            match split_lines_into_vectors(lines) {
                Ok((mut vec_a, mut vec_b)) => {
                    sort_vectors(&mut vec_a, &mut vec_b);
                    differences = vec_a
                        .iter()
                        .zip(vec_b.iter())
                        .map(|(&a, &b)| calculate_diff(a, b))
                        .collect();

                }
                Err(e) => eprintln!("Error splitting lines: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("\nAnswer: {:?}", sum_of_vector(differences));

}
