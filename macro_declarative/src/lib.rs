// https://veykril.github.io/tlborm/decl-macros/minutiae/fragment-specifiers.html
// block : {} 괄호로 구분된 블록에 매칭됨.
// expr : 모든 종류의 표현식에 매칭됨.
// ident : 식별자 or 키워드에 매칭됨.
// item : 가시성 수식어가 포함된 아이템 조각과 매칭됨. ex) struct Foo; impl Foo {} ...
// lifetime : 라이프타임 조각은 라이프타임 또는 레이블에 매칭됨. 식별자과 상당히 비슷하지만 앞에 '가 붙어 있음.
// literal : 모든 리터럴 표현식에 매칭됨. (https://doc.rust-lang.org/reference/expressions/literal-expr.html)
// meta : 메타는 속성의 내용과 매칭됨. 즉, 일반적인 인수가 없는 간단한 경로와 구분된 토큰 트리 또는 = 뒤에 리터럴 식이 뒤따름. (https://doc.rust-lang.org/reference/attributes.html)
// pat : 2021년 에디션부터 시작되는 오어 패턴을 포함하여 모든 종류의 패턴과 매칭됨. (https://doc.rust-lang.org/reference/patterns.html)
// pat_param : 2021년 판에서는 pat fragment 유형에 대한 동작이 or-패턴을 파싱할 수 있도록 변경됨. 이렇게 하면 해당 조각의 후속 목록이 변경되어 해당 조각이 | 토큰으로 이어지는 것을 방지할 수 있음. 이 문제를 방지하거나 이전 조각 동작을 되돌리려면 pat_param 조각을 사용하면 최상위 수준이나 패턴을 허용하지 않으므로 |를 따를 수 있음.
// path : TypePath 스타일 경로와 매칭됨. 여기에는 함수 스타일 특성 양식인 Fn() -> (https://doc.rust-lang.org/reference/paths.html#paths-in-types)이 포함됨.
// stmt : 하나가 필요한 항목 문(예: Unit-Struct)이 아닌 한, 뒤에 오는 세미콜론이 없는 문들과 매칭됨.
// tt : 토큰트리와 매칭됨.
// ty : 모든 유형의 식과 매칭됨. (https://doc.rust-lang.org/reference/types.html#type-expressions)
// vis : 비어 있을 수 있는 Visibility 한정자와 일치함. (https://doc.rust-lang.org/reference/visibility-and-privacy.html)

// #[macro_export]
// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

extern crate macro_trait;

// #[macro_export]
// macro_rules! my_call {
//     ($var:ident as $ty:ty) => {
//         {
//             macro_trait::Parse::parsing::<$ty>($var);
//         }
//     };
// }

#[macro_export]
macro_rules! my_string {
    ($s:literal) => {
        {
            String::from($s)
        }
    };
}

#[macro_export]
macro_rules! my_min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    }
}