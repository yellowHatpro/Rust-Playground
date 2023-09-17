pub mod panics {
    use std::{fs::File, io::ErrorKind};

    pub fn panics() {
        // If program fails or cannot be handled gracefully, we can call panic macro which will quit the program immediately
        panic!("crash and burn!");
    }

    pub fn result_enums() {
        // just like option enum
        pub enum Result<T, E> {
            Ok(T),
            Err(E),
        }
        let f = File::open("chapter9.rs");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("chapter9.rs") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                _ => panic!("Could not open file due to error: {}", error),
            },
        };
        println!("{:#?}", f);
    }
    pub fn error_propagation() {
        //I died here
    }
}
