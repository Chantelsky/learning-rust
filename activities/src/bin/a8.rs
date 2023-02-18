// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor {
    Cola,
    Sprite,
    Lemonade
}

struct Drink {
    flavor: Flavor,
    litres: f64,
}

fn print_drink(drink:Drink) {
    match drink.flavor {
        Flavor::Cola => println!("Cola"),
        Flavor::Sprite => println!("Sprite"),
        Flavor::Lemonade => println!("Lemonade")
    }

    println!("litres: {:?}", drink.litres)
}

fn main() {
    let cola = Drink {
        flavor: Flavor::Cola,
        litres: 1.25
    };

    print_drink(cola);
}
