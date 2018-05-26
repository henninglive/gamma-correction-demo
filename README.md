# gamma-correction-demo
Integrative demo of a gamma curve implemented using Rust and SDL2.
Press up/down to change the gamma factor.

![](https://github.com/henninglive/gamma-correction-demo/blob/master/example.png?raw=true)

#### Build Requirements
 - SDL2
 - pkg-config

#### Build and Run
1. Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org/) installed
2. Clone the project `$ git clone https://github.com/henninglive/gamma-correction-demo/ && cd gamma-correction-demo`
3. Build the project `$ cargo build --release` (NOTE: There is a large performance differnce when compiling without optimizations, so I recommend alwasy using `--release` to enable to them)
4. Once complete, the binary will be located at `target/release/gamma-correction-demo`
5. Use `$ cargo run --release` to build and then run, in one step
