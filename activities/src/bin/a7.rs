// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Color {
  Pink,
  Purple,
  Blue,
  Green
}

fn print_color(my_color: Color) {
  match my_color {
    Color::Pink => println!("Pink"),
    Color::Purple => println!("Purple"),
    Color::Blue => println!("Blue"),
    Color::Green => println!("Green")
  }
}

fn main() {
  print_color(Color::Pink);
}
