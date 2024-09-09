use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn read_username_from_file_long() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    // panic! macro
    // panic!("crash and burn");

    // Index out-of-bounds access will cause panic
    // let v = vec![1, 2, 3, 4];
    // v[99];

    // Use Result<T, E> for recoverable error handling
    let path = "hello.txt";
    let project_readme_result = File::open(&path);
    let mut project_readme_file = match project_readme_result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(&path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other => panic!("Problem opening the file: {other:?}"),
        },
    };

    // Alternatively, using `unwrap_or_else`, removes nested match statements
    let _ = File::open(&path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    let mut buf = String::new();
    let read_result = project_readme_file.read_to_string(&mut buf);
    let _ = match read_result {
        Ok(_) => (),
        Err(e) => panic!("Problem reading file: {e:?}"),
    };
    println!("file contents: {buf}")
}
