
/// 1. String Slice (&str)
///
/// A string slice is a reference to a string, typically borrowed.
/// It is immutable by default and points to a sequence of UTF-8-encoded characters.
/// Commonly used for string literals or references to parts of a String.
///
/// Key Characteristics:
///
/// Immutable: Cannot be changed once created.
/// Efficient: Does not allocate new memory; it points to existing data.
/// Fixed size: Known size at compile time.
/// Stored on the stack when referencing literals or as part of larger data structures.
///
/// Common Usage:
///
///    Function arguments (&str is often used to avoid ownership transfer).
///    String literals like "Hello, world!" are &str.
pub fn string_slice() -> () {
    let literal: &str = "Hello &str, world!"; // String literal
    let slice: &str = &literal[0..10];    // Substring
    println!("{}", slice);              // Output: Hello
}

/// 2. Owned String (String)
///
///     A String is a heap-allocated, growable, and mutable string type.
///     It owns its data, meaning it has exclusive control over the memory it occupies.
///
/// Key Characteristics:
///
///    Mutable: You can change its contents (e.g., add or remove characters).
///    Growable: Can dynamically allocate more memory when needed.
///    Stored on the heap.
///
/// Common Usage:
///
///     When you need ownership of a string.
///     For dynamically constructed or modified strings.

pub fn owned_string() -> () {
    let mut s: String = String::from("Hello");
    s.push_str(", world!");
    s.push_str(" Owned String");
    println!("{}", s); // Output: Hello, world! Owned String
}

/// ### Differences Between `&str` and `String`
///
/// |Feature|`&str` (String Slice)|`String` (Owned String)|
/// |---|---|---|
/// |**Memory**|Stack (or borrowed from heap)|Heap|
/// |**Ownership**|Borrowed|Owned|
/// |**Mutability**|Immutable|Mutable|
/// |**Size**|Fixed at compile-time|Dynamic (can grow/shrink)|
/// |**Usage**|String literals, read-only operations|Modifiable, dynamically created strings|
///
/// ---
///
/// ### Conversion Between `String` and `&str`
/// 1. **From `&str` to `String`**: Use `String::from` or `.to_string()`:
/// 2. **From `String` to `&str`**: Use a reference or `.as_str()`:
pub fn conversion_between_string_and_str() -> () {
    let slice: &str = "Hello from str to String";
    let owned: String = slice.to_string();
    println!("{}", owned);

    let owned_2: String = String::from("Hello from String to str");
    let slice_2: &str = &owned_2;
    let slice_3: &str = owned_2.as_str();
    println!("{}", slice_2);
    println!("{}", slice_3);
}

/// CString:
///
///     A string compatible with C-style null-terminated strings.
///     Used for FFI (Foreign Function Interface).

pub fn c_string() -> () {
    use std::ffi::CString;
    let c_string = CString::new("Hello CString").expect("CString::new failed");
    // `CString` does not implement `Display` (required by `{}`): E0277 need to use the macros "{:?}"
    println!("{:?}", c_string);
}

/// OsString and OsStr:
///
///     Platform-specific string types for compatibility with operating systems.
///     Useful when working with file paths or environment variables.

pub fn os_string() -> () {
    use std::ffi::OsString;
    let os_string = OsString::from("Hello OsString");
    // `OsString` does not implement `Display` (required by `{}`): E0277 need to use the macros "{:?}"
    println!("{:?}", os_string);
}


/// Cow<str>:
///
///     A clone-on-write string type.
///     Can either borrow or own its data, optimizing performance.
/// A clone-on-write smart pointer.
/// The type Cow is a smart pointer providing clone-on-write functionality:
/// it can enclose and provide immutable access to borrowed data, and clone the data lazily when
/// mutation or ownership is required. The type is designed to work with general borrowed data via
/// the Borrow trait.
///
/// Cow implements Deref, which means that you can call non-mutating methods directly on the data
/// it encloses. If mutation is desired, to_mut will obtain a mutable reference to an owned value,
/// cloning if necessary.
///
/// If you need reference-counting pointers, note that Rc::make_mut and Arc::make_mut can provide
/// clone-on-write functionality as well.

pub fn cow_string() -> () {
    use std::borrow::Cow;
    let cow: Cow<str> = "Hello Cow<str>".into(); // Borrowed
    println!("{}", cow);
}

