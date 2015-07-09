// this function basically says that it takes in two references and they must be at least the same lifetime (or live longer)
// fn at line 13 just means that it takes a reference to a Car that has some arbitrary lifetime 'a, and it returns a string 
// that has the same lifetime 'a
// so effectively that means that the string it returns lives as long as the car that was passed to it
// which makes sense, since it's returning a reference to a member of the Car

struct Car {
    // `String` is a heap allocated string
    manufacturer: String,
    model: String,
    year: i32,
}

fn get_model<'a>(car: &'a Car) -> &'a str {
    &car.model
}

fn main() {
    let car = Car {
        // construct a `String` from a reference to a string (`&'static str`)
        // by copying of the data
        manufacturer: "Tesla".to_string(),
        model: "Model S P85D".to_string(),
        year: 2015,
    };

    let model: &str = get_model(&car);

    println!("Car model: {}", model);
}