use macro_derive::HelloWorldMacro;
use macro_trait::HelloWorld;

#[derive(HelloWorldMacro)]
struct Test {}

fn main() {
    Test::hello_world();
}