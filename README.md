# nostrum

> [!WARNING]
> Under construction 🚧

No enum or struct

## Overview

In Rust, structs and enums are not objects as in OOP languages. This is fine,
but it may struggle people who have lived in OOP world. For example, to do ideal
TDD, we want to test a specific component of code in isolation, but it cannot be
done with struct or enum. But, with traits, we can. So this crate provides auto
generation of all boilerplate code to use a struct or enum as like an object.

## Usage

```rust
#[nostrum::nostrum]
impl Cat {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn meow(&self) -> String {
        format!("{}: meow", self.name)
    }
}
```

This will produce the following code:

```rust
pub trait CatObj {
    fn name(&self) -> &str;
    fn meow(&self) -> String;
}

impl CatObj for Cat {
    fn name(&self) -> &str {
        &self.name
    }

    fn meow(&self) -> String {
        format!("{}: meow", self.name)
    }
}
```

### Default implementation

You can attach `#[nostrum(default = expr)]` to a method or the whole impl block
to provide a default implementation for the method.
