# Into e From

`into` será como um convertor de tipo e o `from` que o tipo for compativo será convertido 

```rust
struct Numero {
    num: i32
}

impl From<f32> for Numero {
    fn from(num: f64) -> Self {
        Numero { num: num as i32 }
    }
}
```