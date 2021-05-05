fn main() {
    // USING OPTION TYPE TO DEAL WITH ABSENCE

    // The Rust standard library provides an Option<T> enum to be used when the absence of a value is a possibility. Option<T> is widely used in Rust code because it encodes the very common scenario in which a value could be something or it could be nothing.

    // In other languages, this would be modeled using null or nil. Rust doesn't use null outside of code that interoperates with other languages. This means Rust is explicit about when a value is optional while other languages using a function taking a String could take a String or null. Rust it is String only. If we want to model an optional string in Rust we need to explicitly wrap it in an Option type, Option<String>.

    // Option<T> manifests itself as one of two variants:
    // enum Option<T> {
    //     None, // The value doesn't exist
    //     Some(T), // The value exists
    // };

    // The <T> part of the option enum declarations states that the type T is generic and will be associated with the Some variant of the Option enum.

    // As discussed in previous sections, None and Some are not types but rather variants of the Option<T> type. This means among other things that functions cannot take Some or None as arguments but only Option<T>.

    // In the preceding unit, we mentioned that trying to access a vector's non-existent index would cause the program to panic, but you could avoid that by using the Vec::get method, which return an Option type instead of panicking. If the value exists at a specified index, it's wrapped in the Option::Some(value) variant. If the index is out of bounds, it would return a Option::None value instead.

    // Example:
    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // Pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);

    // Pick the third item:
    let third = fruits.get(2);
    println!("{:?}", third);

    // Pick the 99th item, which is non-existent:
    let non_existent = fruits.get(99);
    println!("{:?}", non_existent);

    // The first two prints get us Some("banana") and Some("coconut"). The third one returned a None value (which isn't associated with any data) instead of panicking if we didn't use get.

    // In practice, we must decide how our program behaves depending on what enum variant it gets. Now on how to access data inside Some(data).

    // PATTERN MATCHING

    // Rust has an extremely powerful control flow operator called match, which we can use to compare a value against a series of patterns and then execute code based on which pattern matches.

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // Explanation for '&' from https://stackoverflow.com/questions/57339201/what-is-the-purpose-of-before-the-loop-variable
    // So normally when you use for i in list, the loop variable i would be of type &i32.
    // But when instead you use for &i in list, you are not dereferencing anything, but instead you are using pattern matching to explicitly destructure the reference and that will make i just be of type i32.
    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(fruit_name) => println!("it's a delicious {}", fruit_name),
            None => println!("There is no fruit! :("),
        }
    };

    // In the above code we iterate through the same indexes from our previous example (0, 2, and 99) and then use each one to retrieve a value from the fruits vector by using the fruits.get(index) expression.

    // Because the fruits vector contains &str elements, we know that the result of this expression is of type Option<&str> (I believe this is why we need & before our variable index so that we are explicitly matching the elements &index and &str). We then use a match expression against the Option value and define a course of action for each of its variants. Rust refers to those branches as match arms, and each arm can handle on possible outcome for the matched value.

    // We can refine our match expression even further to act differently, depending on the values inside a Some variant. For example, we could stress the fact the coconuts are awesome by running the following:

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are awesome!!!"),
            Some(fruit_name) => println!("It's a delicious {}", fruit_name),
            None => println!("there is no fruits! :("),
        };
    };


}
