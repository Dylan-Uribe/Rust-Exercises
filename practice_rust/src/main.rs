
fn is_even(number: i32) -> bool {
    number % 2 == 0
}

fn calculate (number_1: i32, number_2: i32){

    let operations: (i32, i32, i32, i32) = (
        number_1 + number_2,
        number_1 - number_2,
        number_1 * number_2,
        number_1 / number_1);

    println!("operations: {:?}", operations);
}

fn main() {

    //Define a function to know if a number is even.

    println!("FIRST EXERCISE");
    let number_1 = 3;
    let number_2 = 4;
    let number_3 = 7;
    let number_4 = 8;

    println!("Is {} even? {}", number_1, is_even(number_1));
    println!("Is {} even? {}", number_2, is_even(number_2));
    println!("Is {} even? {}", number_3, is_even(number_3));
    println!("Is {} even? {}", number_4, is_even(number_4));

    //Define a calculate function that takes two i32 numbers and return
    // the fourth basic operations

    println!("\nSECOND EXERCISE");
    let number_1 = 5;
    let number_2 = 10;

    calculate(number_1, number_2);

    //An array
    println!("\nTHIRD EXERCISE");
    println!("ARRAY");
    let array = [1,2,3,4,5];

    for element in array {
        println!("The value is {}", element);
    }

    //BASIC properties
    println!("\nFOURTH EXERCISE"); println!("BASIC PROPERTIES");
    let mut basic_property = "this a &str variable";
    println!("Basic: {}", basic_property);
    basic_property = "change value of &str variable, it is mutable";
    println!("Basic: {}", basic_property);
    let mut basic_property_2 = String::from("This is a String variable");
    basic_property_2.push('!');
    println!("Basic: {}", basic_property_2);
    basic_property_2 = String::from("Change the String variable because it is mutable");
    println!("Basic: {}", basic_property_2);
}


