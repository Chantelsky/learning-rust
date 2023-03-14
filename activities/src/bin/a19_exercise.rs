use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents {content: "Stuff".to_owned()});
    lockers.insert(2, Contents {content: "air pods".to_owned()});
    lockers.insert(3, Contents {content: "protein shake".to_owned()});

    for (locker_number, content) in lockers.iter() {
        println!("number: {:?}, content: {:?}", locker_number, content);
    }
}