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
}
