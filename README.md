`Iterator::enumerate`, but use u32, i64, f32 etc

# Examples
```rust
use enumerate_number::EnumerateNumber as _;

let mut iter = "foo".chars().enumerate_f64();
assert_eq!(iter.next(), Some((0.0, 'f')));
assert_eq!(iter.next(), Some((1.0, 'o')));
assert_eq!(iter.next(), Some((2.0, 'o')));
assert_eq!(iter.next(), None);
```
