
In **Rust**, the following visibility levels are available for module elements (functions, structs, methods, constants, etc.):

### 1. **Private** (default)

- By default, elements declared without a visibility keyword are **private**.
- Private elements are accessible only within the current module (and its submodules).
- Example:

```rust
mod my_module {
    fn private_function() {
        println!("This is a private function");
    }
}
```

### 2. **Public** (`pub`)

- Elements marked with the `pub` keyword are accessible outside the current module.
- This allows "exporting" elements for use in other modules or external code.
- Example:

```rust
mod my_module {
    pub fn public_function() {
        println!("This is a public function");
    }
}
```

### 3. **Restricted (`pub(crate)`)**

- Visibility is restricted to the current **crate** (library or binary).
- Such elements are accessible anywhere within the project (crate) but not outside of it.
- Example:

```rust
mod my_module {
    pub(crate) fn crate_function() {
        println!("This function is visible within the crate");
    }
}
```

### 4. **Module-Restricted Visibility (`pub(super)` and `pub(in)`)**

#### `pub(super)`

- Visibility is restricted to the **parent module**.
- Example:

```rust
mod parent {
    pub mod child {
        pub(super) fn parent_visible_function() {
            println!("This function is visible in the parent module");
        }
    }
}
```

#### `pub(in <path>)`

- Visibility is restricted to a specific **module or path**.
- `<path>` specifies the module where the item is visible.
- Example:

```rust
mod parent {
    pub mod child {
        pub(in crate::parent) fn restricted_function() {
            println!("This function is visible only in the 'parent' module");
        }
    }
}
```

### 5. **Inherited Visibility**

- Methods, struct fields, and other nested items **inherit visibility** from their parent item.
- For struct fields:
    - If the struct is `pub`, but its fields are **not marked as `pub`**, the fields remain private.
    - Example:

```rust
pub struct PublicStruct {
    pub public_field: i32,
    private_field: i32, // Accessible only within the module
}
```

### Summary Table of Visibility Levels

| Visibility Keyword | Scope of Access                                  |
| ------------------ | ------------------------------------------------ |
| **(default)**      | Only within the current module                   |
| `pub`              | Accessible from any module                       |
| `pub(crate)`       | Accessible only within the current crate         |
| `pub(super)`       | Accessible only in the parent module             |
| `pub(in <path>)`   | Accessible only within the specified module/path |

These visibility levels allow precise control over access to items, enabling proper encapsulation and modular design in Rust.