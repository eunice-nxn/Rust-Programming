use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //Unrecoverable errors with panic
    //panic!("crash and burn");

    let v = vec![1, 2, 3];
    
    v[99]; // Using [] is supposed to return an element.

    //Recoverable errors with Result
    // File::open function returns Result<T, E>
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let f2 = File::open("hello.txt");
    let f2 = match f2 {
        Ok(file) => file,
        // Err variant is io::Error, which is a struct provided by the standart library
        Err(error) => match.error_kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e), 
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    let f3 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind == ErrorKind::NotFound{
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


    // unwrap method is a shortcut method that is implemented just like the match expression
    let f4 = File::open("hello.txt").unwrap();

    // expect method let us also choose the panic! error message
    let f5 = File::open("hello.txt").expect("Failed to open hello.txt");



}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

/* ? operator calls from function of From trait in the standard library
 which is used to convert errors from one type into another
 As long as each error type implements the from function to define
 how to convert itself to the returned error type, the ? operator takes care of the conversion automatically */
 fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt");
}

