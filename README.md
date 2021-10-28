# squirrel3-rs

Fast, simple and reliable; noise-based, random number generator.  

Using; [squirrel3](https://www.youtube.com/watch?v=LWFzPP8ZbdU) hashing function for the noise.

## Usage
```rust
[dependencies]
squirrel3-rs = "0.1.0"
```

```rust
use squirrel3_rs::sq3;

fn main() {
    println!("{}", sq3::rand());
}
```

## Examples

### Basic:

```rust
println!("True or False => {}", sq3::bool());
println!("[0.0, 1.0) => {}", sq3::rand());
println!("[0, 100) => {}", sq3::u32(100));
```
### Range:

```rust
println!("[2.14, 7.21) => {}", sq3::range_f32(2.14, 7.21));
println!("[-540, 2160) => {}", sq3::range_i32(-540, 2160));
// And more...
```
### Array:

```rust
let mut fruits = vec!["apple", "banana", "peach", "orange", "strawberry"];
sq3::shuffle(&mut fruits);

println!("Shuffled: {:?}", fruits);
println!("I want to eat: {}", fruits[ sq3::usize(fruits.len()) ] );
```
### Local State:
```rust
// Using manually created state, instead of thread_local.
let rng = sq3::Rng::new(2021 /*seed*/);
println!("True or False => {}", rng.bool());
```

### Stateless Functions: ((No side effects))
```rust
// Original squirrel3
println!("[0, u32::MAX) => {}", sq3::squirrel3(3 /*position*/, 4 /*seed*/));
println!("[0, 1.0) => {}", sq3::normalized(3 /*position*/, 4 /*seed*/));

// Noise functions, returns a value between [0.0, 1.0)
println!("|x: 100| => {}", sq3::noise1d(100));
println!("|x: 100, y: 200| => {}", sq3::noise2d(100, 200));
println!("|x: 100, y: 200, z: 300| => {}", sq3::noise3d(100, 200, 300));
```

## License
[MIT](https://choosealicense.com/licenses/mit/)