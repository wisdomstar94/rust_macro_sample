use macro_attribute::my_custom_attribute;
use macro_derive::HelloWorldMacro;
use macro_trait::HelloWorld;

// "절차적 매크로 - 파생 매크로"는 derive 와 함께 구조체에서만 사용 가능! (ex. struct, enum)
// 현재 scope 에 존재하는 특정 trait 에 대한 기본 구현을 해준다.
#[derive(HelloWorldMacro)]
struct Test {}

// "절차적 매크로 - 속성 매크로"는 함수의 내용을 특정 내용으로 변경시켜준다.
// 예를 들어 기존 함수 내부 코드의 전, 후로 특정 코드를 추가하는 것도 가능하다.
#[my_custom_attribute]
fn my_function() {
    println!("why..?");
} 

fn main() {
    Test::hello_world();
    my_function(); 
}