In Rust, Rc (reference counting) is a smart pointer that enables shared ownership of data with reference counting. 
It is useful when multiple parts of your code need to own the same data, and the data is immutable.
Key Characteristics of Rc:

    Used for shared ownership.
    Works only in single-threaded applications (for multithreaded scenarios, use Arc).
    The data referenced by Rc is immutable by default. To enable mutability, you can use Rc<RefCell<T>>.

Here, RefCell enables interior mutability within the Rc, 
allowing the data to be modified while still following Rust's borrowing rules at runtime.

When to Use Rc?

    When you need shared ownership of immutable data in a single-threaded context.
    For complex data structures, such as graphs or trees, where nodes have multiple parents.
    When mutability of data is needed within Rc, you can use Rc<RefCell<T>>.

When NOT to Use Rc?

    In multithreaded applications (use Arc instead for thread-safe shared ownership).
    If ownership can be expressed with references or Box, as these are simpler and faster.