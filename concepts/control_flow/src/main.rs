fn main() {
    // // Expression if
    // let number = 5;
    // if number % 4 == 0 {
    //     println!("Number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("Number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("Number is divisible by 2");
    // } else {
    //     println!("Number is not divisible by 4, 3, or 2");
    // }
    // // if in a let statement
    // // the value of the expression must be the same type as the type of the let statement
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("The value of number is: {}", number);

    // loop

    // repetition with loops
    // loop {
    //     println!("again!");
    // };

    // // return value from loop
    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter*2;
    //     }
    // };
    // println!("The result is {result}");

    // // loop labels
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count +=1;
    // }
    // println!("End count = {count}");

    // while
    let mut number = 3;
    while number != 0 {
        println!("{number} !");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // looping each element of a collection with while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    // looping through a collection with for
    println!("looping through a collection with for:");
    let b = a;
    for element in b {
        println!("the value is {}", element);
    }

    for number in (1..=3).rev() {
        println!("{number} !");
    }
    println!("LIFTOFF!!!");

}
