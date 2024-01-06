use macro_declarative::my_min;
use macro_trait::HelloWorld;

// struct A {

// }

struct B {

}

impl HelloWorld for B {
  fn hello_world() {
      println!("hello~~ B~~");
  }
}

#[test]
fn macro_declarative_test() {
  // let b = B {};
  // let k = my_parse!(b as A);
  let min = my_min!(3, 10);
  dbg!(min);
}