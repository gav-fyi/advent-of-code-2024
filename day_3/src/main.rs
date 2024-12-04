use utils::read_string_from_file;
use regex::Regex;


fn part_1() {

    let pattern = r"mul\((\d{1,3}),(\d{1,3})\)";
    let mut rolling_total: i32 = 0;

    match read_string_from_file("data_1.txt") {
        Ok(text) => {
            let re = Regex::new(pattern).unwrap();
            for caps in re.captures_iter(text.as_str()) {
                
                let first: i32 = caps[1].parse().unwrap();
                let second: i32 = caps[2].parse().unwrap();

                rolling_total += first * second;
        
                // println!("Found match: {}, {}", first, second);
            }
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Answer 1: {}", rolling_total);

}

fn part_2() {

    let mul_pattern = r"mul\((\d{1,3}),(\d{1,3})\)";

    let mut rolling_total: i32 = 0;

    match read_string_from_file("data_2.txt") {
        Ok(mut text) => {
            
            let re = Regex::new(mul_pattern).unwrap();

            while text != "" {
                
                // println!("{:?}", text);

                match text.split_once("don't()") {
                    Some((dont_before, dont_after)) => {

                        for caps in re.captures_iter(&dont_before) {
                    
                            let first: i32 = caps[1].parse().unwrap();
                            let second: i32 = caps[2].parse().unwrap();
            
                            rolling_total += first * second;
                    
                            // println!("Found match: {}, {}", first, second);
                        }

                        // println!("DONT");
                        // println!("{:?}", dont_after);

                        match dont_after.split_once("do()") {
                            Some((_, do_after)) => {
                                // println!("DO");
                                // println!("{:?}", do_after);
                                text = do_after.to_string();
                            },
                            None => {
                                text = "".to_owned();
                            }
                        }
                    },
                    None => {
                        for caps in re.captures_iter(&text) {
                    
                            let first: i32 = caps[1].parse().unwrap();
                            let second: i32 = caps[2].parse().unwrap();
            
                            rolling_total += first * second;
                    
                            // println!("Found match: {}, {}", first, second);
                        }
                        text = "".to_owned();
                    }
                }
            }
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }

    println!("Answer 2: {}", rolling_total);
}

fn main() {
    part_1();
    part_2();
}