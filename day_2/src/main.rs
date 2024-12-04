use utils::read_lines_from_file;

fn parse_ints_from_string(input: &str) -> Result<Vec<i32>, &'static str> {
    input
        .split_whitespace()
        .map(|s| s.parse::<i32>().map_err(|_| "Failed to parse an integer"))
        .collect()
}

fn calculate_distances(numbers: &Vec<i32>) -> Vec<i32> {
    numbers
        .windows(2)
        .map(|pair| (pair[1] - pair[0]).abs())
        .collect()
}

fn get_all_permutations(line: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::new();
    for (index, _) in line.iter().enumerate() {
        let mut temp: Vec<i32> = line.clone();
        temp.remove(index);
        ret.push(temp);
    }
    ret
}

fn is_line_safe(level: &Vec<i32>, allow_bad_level: bool) -> bool {
    let mut sorted = level.clone();
    sorted.sort();
    let mut rev_sorted = level.clone();
    rev_sorted.sort_by(|a, b| b.cmp(a));
    if *level == sorted || *level == rev_sorted {
        let distances = calculate_distances(&level);
        let max = distances.iter().max();
        let min = distances.iter().min();
        if let Some(max_value) = max {
            if let Some(min_value) = min {
                if *min_value > 0 && *max_value < 4 {
                    return true;
                }
            }
        }
    }

    if allow_bad_level {
        for diff in get_all_permutations(&level) {
            if is_line_safe(&diff, false) {
                return true;
            }
        }
    }

    false
}

fn part_1() {

    let mut safe_count: i32 = 0;

    match read_lines_from_file("data_1.txt") {
        Ok(lines) => {
            for line in lines {
                match parse_ints_from_string(&line) {
                    Ok(level) => {
                        if is_line_safe(&level, false) {
                            safe_count +=1;
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Answer 1: {}", safe_count);

}

fn part_2() {

    let mut safe_count: i32 = 0;

    match read_lines_from_file("data_2.txt") {
        Ok(lines) => {
            for line in lines {
                match parse_ints_from_string(&line) {
                    Ok(level) => {
                        if is_line_safe(&level, true) {
                            safe_count +=1;
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Answer 2: {}", safe_count);
}

fn main() {
    part_1();
    part_2();
}