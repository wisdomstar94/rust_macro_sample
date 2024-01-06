use macro_attribute::my_custom_attribute;
use macro_declarative::my_string;
use macro_derive::HelloWorldMacro;
use macro_trait::HelloWorld;

// "절차적 매크로 - 파생 매크로(커스텀 derive 매크로)"는 derive 와 함께 구조체에서만 사용 가능! (ex. struct, enum 등)
// 현재 scope 에 존재하는 특정 trait 에 대한 기본 구현을 해준다.
#[derive(HelloWorldMacro)]
#[attr1(name = "hong", scores = [1, 2, 3])]
#[attr2(GET, 404, "/")]
struct Test {}

// "절차적 매크로 - 속성 매크로"는 함수의 내용을 특정 내용으로 변경시켜준다.
// 예를 들어 기존 함수 내부 코드의 전, 후로 특정 코드를 추가하는 것도 가능하다.
#[my_custom_attribute(GET, ",_,_,", 33, 0.5, [1, 2, 3])]
// #[my_custom_attribute(name = "honggildong", scores = [1, 2, 3])]
fn my_function(a: &u32) { 
    println!("my_function call! {}", a); 
}

// struct TT {

// }

// impl HelloWorld for TT {
//     fn hello_world() {
//         println!("TT hello_world~~~");
//     }
// }

fn main() {
    Test::hello_world();
    let a = 13;
    my_function(&a); 
    println!("{}", my_string!("nice~"));

    // let tt = TT {};
    // my_call!(tt as TT);
}