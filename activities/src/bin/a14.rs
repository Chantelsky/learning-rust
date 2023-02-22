// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:

// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print(data: &str) {
    println!("{:?}", data);
}

// * The name and colors should be printed using a function
fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 5,
            name: String::from("Apollo"),
            color: String::from("Purple"),
        },
        Person {
            age: 10,
            name: String::from("Shadow"),
            color: String::from("Pink"),
        },
        Person {
            age: 15,
            name: String::from("Yoda"),
            color: String::from("Green"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in people {    
    // * Use an if expression to determine which person's info should be printed
        if person.age <= 10 {
            print(&person.name);
            print(&person.color);
            // println!("name: {:?}, color: {:?}",person.name, person.color);
        }
    }
}
