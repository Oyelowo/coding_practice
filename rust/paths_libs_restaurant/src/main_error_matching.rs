use std::{error::Error, fs::{self, File}, io::{self, ErrorKind}};

fn main() {
    // let v = vec![1, 2, 3];
    // error_pattern_matching();
    // error_callback();
    // read_username_from_file();
}

fn main3() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn read_username_from_file_brief2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_brief() -> Result<String, io::Error> {
    let f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn error_pattern_matching() {
    // v[99];
    // crash_it();
    let filename = "hellowr.txt";

    let f = File::open(&filename);
    /*     let f = match f {
        Ok(f) => f,
        Err(err) => panic!("Oti baje {:?}", err),
    }; */
    let _f = match f {
        Ok(f) => f,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(&filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

fn error_callback() {
    // Shortcuts for Panic on Error: unwrap and expect
    let filename = "hellowr.txt";

    // let f = File::open(filename).unwrap();
    // let f = File::open(filename).expect("Bad error from here");
    let f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).unwrap_or_else(|err| {
                panic!("Problem creating file, {:?}", error);
            })
        } else {
            panic!("Problem opening file, {:?}", error);
        }
    });
}

fn crash_it() {
    panic!("crash and burn")
}
