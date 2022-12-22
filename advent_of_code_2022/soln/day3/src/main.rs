fn main() {
    println!("Hello, world!");
}

struct EachInstance {
    first: String,
    second: String,
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

fn extract_data (file_data: & String) -> Vec<EachInstance> {
    
}
