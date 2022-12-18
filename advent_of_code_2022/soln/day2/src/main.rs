use std::io::prelude::*;
use std::fs::File;
use std::io::ErrorKind;

struct EachInput {
    opponent: i8,
    you: i8,
}

fn main() {
    let mut file_data = String::new();
    file_test(&"DropHere/data.txt".to_string()).read_to_string(& mut file_data)
        .expect("Failed to read the file");

    file_data.push('\n');

    let result = extract_data(&file_data);
    let score = score_calc(&result);
    println!("the score is: {}", score);
}

fn file_test (source_dir: & String) -> File {
    return match File::open(source_dir) {
        Ok(fc) => fc,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            std::fs::create_dir_all("DropHere").expect("cannot create the drop folder");
            File::create(source_dir).expect("cannot create the file");
            std::fs::write(source_dir, "test").expect("File is protected cannot write data");
            file_test(source_dir)
        },
        Err(_error) => {
            panic!("unkown issue");
        }
    };
}

fn extract_data(file_data:& String) -> Vec<EachInput> {
    let mut vec_all: Vec<EachInput> = Vec::new();

    let mut opponent: i8 = 0;
    
    let mut you: i8 = 0;

    for character in file_data.chars() {
        match character {
            'A' => {
                opponent = 1;
            },
            'B' => {                
                opponent = 2;
            },
            'C' => {
                opponent = 3;
            },
            'X' => {
                you = 1;
            },
            'Y' => {
                you = 2;
            },
            'Z' => {
                you = 3;
            },
            '\n' => {
                let data = EachInput {
                    opponent: opponent,
                    you: you,
                };
                vec_all.push(data);
            },
            _ => {},
        };
    }
    
    return vec_all;
}

/*
 * the following are the possibility can might appear
 * 1 rock vs 2 paper 2-1 = 1 win   1-2 =-1 lose
 * 2 paper vs 3 sissor 3-2 = 1 win 2-3 =-1 lose
 * 3 sissor vs 1 rock 1-3 = -2 win 3-1 =2 win
 */
fn score_calc (data_info:& Vec<EachInput>) -> i64 {
    let mut score: i64 = 0;
    for item in data_info {
        if item.you - item.opponent == 1 || item.you - item.opponent == -2 {
            score += 6;
            score += item.you as i64;
        } else if item.you == item.opponent {
            score += 3;
            score += item.you as i64;
        } else {
            score += item.you as i64;
        }
    }

    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_data() {
        let mut test = extract_data(&"A Y\nC X\nB Z\n".to_string());
        let mut index = 1;
        for item in &test {
           if index == 1 {
               println!("part1");
               assert_eq!(1, item.opponent);
               assert_eq!(2, item.you);
           } else if index == 2 {
               println!("part2");
               assert_eq!(3, item.opponent);
               assert_eq!(1, item.you);
           } else if index == 3 {
               println!("part3");
               assert_eq!(2, item.opponent);
               assert_eq!(3, item.you);
           }
           index += 1;
        }
    }

    #[test]
    fn correct_score() { 
        let mut file_data = String::new();
        file_test(&"DropHere/example_test.txt".to_string()).read_to_string(& mut file_data)
            .expect("Failed to read the file");

        file_data.push('\n');

        let result = extract_data(&file_data);
        let score = score_calc(&result);
        assert_eq!(15, score);
    }
}
