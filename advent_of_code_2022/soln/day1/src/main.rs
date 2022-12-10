use std::fs::File;
use std::io::ErrorKind;

struct max_info {
    order: i32,
    max_number: u64,
}

fn main() {
    let fs_detail = File::open("DropHere/data.txt");
    
    let file_info = match fs_detail {
        Ok(fc) => fc,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            std::fs::create_dir("DropHere").expect("cannot create the drop folder");
            File::create("data.txt").expect("cannot create the file");
            std::fs::write("DropHere/data.txt", "test");
            std::process::exit(0);
        }
        Err(error) => {
            panic!("unkown issue");
        }
    };

    let info = std::fs::read_to_string("DropHere/data.txt").expect("failed to read the file");

    let mut sum = 0;
    let mut index = 1;

    let mut slice_info = info.lines();

    let mut answer = max_info{order: 0, max_number: 0};
    let mut second_info = max_info{order: 0, max_number: 0};
    let mut third_info = max_info{order: 0, max_number: 0};

    loop {

        let line = slice_info.next();

        let current_line = match line{
            Some(string) => string,
            None => {
                println!("At index {}, the sum is {}", index, sum);
                index += 1;
                sum = 0;
                println!("This is the end of the line");
                break;
            }
        };

        if current_line.trim() == "" {
            if sum > answer.max_number {
                second_info.order = answer.order;
                second_info.max_number = answer.max_number;

                answer.order = index;
                answer.max_number = sum;
            } else if sum > second_info.max_number {
                third_info.order = second_info.order;
                third_info.max_number = second_info.max_number;

                second_info.order = index;
                second_info.max_number = sum;
            } else if sum > third_info.max_number {
                third_info.order = index;
                third_info.max_number = sum;
            } else {
            }
            println!("At index {}, the sum is {}", index, sum);
            index += 1;
            sum = 0;
        } else {
            sum += current_line.trim().parse::<u64>().expect("parse Err");
            println!("{}", sum);
        }
    }

    println!("The max value is {}, with index {}", answer.max_number, answer.order);
    println!("The second max value is {}, with index {}", second_info.max_number, second_info.order);
    println!("The third max value is {}, with index {}", third_info.max_number, third_info.order);
    println!("The sum value is {}", answer.max_number+second_info.max_number+third_info.max_number);
}
