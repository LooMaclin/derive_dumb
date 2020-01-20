## Description

Derive `Dumb` generates a structure in which all fields are public. 
Original structure can be converted using function call `dumb`.

## Example 
```rust
#[derive(Dumb)]
pub struct A {
    a: String,
}
```

Generates:

```rust
pub struct DumbA {
    pub a: String,
}
impl A {
    fn dumb(self) -> DumbA {
        DumbA {
            a: self.a,
        }
    }
}
```