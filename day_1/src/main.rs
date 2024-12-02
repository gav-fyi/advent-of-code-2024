use utils::read_lines_from_file;
use utils::split_lines_into_vectors;
use utils::sort_vectors;
use utils::calculate_diff;
use utils::sum_of_vector;


fn part_1() {

    let mut differences: Vec<i32> = Vec::new();

    match read_lines_from_file("data_1.txt") {
        Ok(lines) => {
            match split_lines_into_vectors(&lines) {
                Ok((mut vec_a, mut vec_b)) => {
                    sort_vectors(&mut vec_a, &mut vec_b);
                    differences = vec_a
                        .iter()
                        .zip(vec_b.iter())
                        .map(|(&a, &b)| calculate_diff(&a, &b))
                        .collect();

                }
                Err(e) => eprintln!("Error splitting lines: {}", e),
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Answer 1: {:?}", sum_of_vector(&differences));

}

fn part_2() {

    let mut scores: Vec<i32> = Vec::new();

    match read_lines_from_file("data_2.txt") {
        Ok(lines) => {
            match split_lines_into_vectors(&lines) {
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

    println!("Answer 2: {:?}", sum_of_vector(&scores));
}

fn main() {
    part_1();
    part_2();
}