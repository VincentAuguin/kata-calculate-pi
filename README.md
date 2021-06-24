# Kata: Estimate Pi With Random Numbers

Write a function that generate a random float number in range [0;1). 🙂

Easy.

Now, estimate the number Pi. 🙃

## Suggested Steps

1. Write a function which return a random float number in range [0;1)
2. Write a function which return a 2D-Point with random coordinates
3. Write a function which generate N points. Then count the number of points inside the circle defined by:
    - the center C(0;0)
    - the radius r=1.0
4. Knowing that:
    - the area of a circle is defined by `πr²` (= number of points in the circle)
    - the area of a circle's square is defined by `2r²` (= number of points in the square = total number of points)

   Write a function which, given a number of points to generate `n`, return the estimation of the number Pi (you may
   give `n >= 1e4` to get enough points)
5. ~~Tell everyone you're the Boss~~ Share with your friends and colleagues

## Code it with [TDD](https://en.wikipedia.org/wiki/Test-driven_development)

1. ❌ Write a failing test
2. ✅ Make it pass
3. 🧩 Refactor your code without breaking the tests
4. 🔄 Repeat from step 1. until it's **done**

## Code it with Rust

1. Install `rustup` (https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Edit with your favorite IDE

- **IntelliJ** official [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust)
- **VSCode** official [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- Others: 🤷 *(make a PR)*

3. Test your code

```bash
cargo test
# or
cargo t
```

4. Run your code

```bash
cargo run
# or
cargo r
```

## All you need from your web browser:

- The [Rust documentation](https://doc.rust-lang.org/rust-by-example/index.html)
- The [crate registry](https://crates.io/)

## Inspiration

Thanks to **Joma Tech** for the [video](https://www.youtube.com/watch?v=pvimAM_SLic) on the subject.
