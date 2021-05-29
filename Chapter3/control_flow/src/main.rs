use::std::io;

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 }; // if expression can be put on the right side of a let statement, these if/else arms must have the same value type
                                                // i.e. you can't have ```let number = if condition { 5 } else { 'six' }```

    println!("The value of number is : {}", number);

    //loop {
    //    println!("Looping...");
    //}

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //value output to result when breaking
        }
    };

    println!("The result is {}", result);


    let mut nummy = 5;
    while nummy != 0 {
        println!("{}!", nummy);

        nummy -= 1;
    }
    println!("LIFTOFF!!!!");


    let a = [5,4,3,2,1];

    for element in a.iter() {
        println!("{}!", element);
    }
    println!("LIFTOFF!!!");


    for element in (1..5).rev() {
        println!("{}!", element);
    }
    println!("Liftoff!!")

}
