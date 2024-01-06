// extern crate proc_macro;
use proc_macro;
use proc_macro2;
use quote::{quote, ToTokens};
use syn::{self, Attribute};

#[derive(Debug)]
enum MyArgData {
    RawStr(String),
    String(String),
    Int(u32),
    Float(f64),
    VecInteger(Vec<u32>),
    VecFloat(Vec<f64>),
    VecString(Vec<String>),
}

enum LiteralType {
    Integer(u32),
    Float(f64),
    String(String),
}

#[derive(Debug)]
struct Attr1 {
    name: Option<String>,
    scores: Option<Vec<u32>>,
}

impl Attr1 {
    fn new_and_parse(vec_attribute: &Vec<Attribute>) -> Self {
        let mut obj = Attr1 {
            name: None,
            scores: None,
        };

        for item in vec_attribute {
            if let Some(t) = item.path().segments.first() {
                if t.ident.to_string().as_str() == "attr1" {
                    let token_stream = item.to_token_stream();
                    if let Some(real_token_stream) = get_named_parsing_real_token_stream(&token_stream) {
                        // dbg!(&real_token_stream);
                        let token_collect: Vec<proc_macro2::TokenTree> = real_token_stream.into_iter().collect();
                        // dbg!(&token_collect);
                        let mut i = 0;
                        for item2 in &token_collect {
                            match item2 {
                                proc_macro2::TokenTree::Ident(v) => { // writed parameter name
                                    // dbg!(v);
                                    match v.to_string().as_str() {
                                        "name" => obj.name = get_string_from_token_tree(token_collect.get(i + 2)),
                                        "scores" => obj.scores = get_vec_integer_from_token_tree(token_collect.get(i + 2)),
                                        _ => {}
                                    }
                                },
                                proc_macro2::TokenTree::Group(_) => {
                                    
                                },
                                proc_macro2::TokenTree::Punct(_) => {
                                    
                                },
                                proc_macro2::TokenTree::Literal(_) => {
                                    
                                },
                            }
                            i = i + 1;
                        }
                    }
                    break;
                }
            }
        }

        obj
    }
}

#[proc_macro_derive(HelloWorldMacro, attributes(attr1, attr2))]
pub fn hello_world(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_world(&ast)
}

fn impl_hello_world(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let attrs = &ast.attrs;

    // attr1 : named parsing
    let attr1_parse = Attr1::new_and_parse(attrs);
    dbg!(attr1_parse);
    
    // attr2 : non-named parsing
    let attr2_parse = parse_attr_non_named(attrs, "attr2");
    dbg!(attr2_parse);

    let gen = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

fn parse_attr_non_named(vec_attribute: &Vec<Attribute>, parse_param: &str) -> Vec<MyArgData> {
    let mut sequential_args: Vec<MyArgData> = Vec::new();

    // dbg!(attrs);
    for item in vec_attribute {
        if let Some(t) = item.path().segments.first() {
            if t.ident.to_string().as_str() == parse_param {
                let token_stream = item.to_token_stream();
                if let Some(real_token_stream) = get_named_parsing_real_token_stream(&token_stream) {
                    // dbg!(&real_token_stream);
                    for item in real_token_stream.into_iter() {
                        match item {
                            proc_macro2::TokenTree::Ident(v) => {
                                sequential_args.push(MyArgData::RawStr(v.to_string()));
                            },
                            proc_macro2::TokenTree::Group(v) => {
                                if let Some(data) = get_group_data(&v) {
                                    sequential_args.push(data);
                                }
                            },
                            proc_macro2::TokenTree::Punct(_) => {
                                // ex) ,
                            },
                            proc_macro2::TokenTree::Literal(v) => { 
                                if let Some(data) = get_literal_data(&v) {
                                    sequential_args.push(data);
                                }
                            },
                        }
                    }
                }
                break;
            }
        }
    }

    sequential_args
}

fn get_literal_type(v: &str) -> Option<LiteralType> {
    let result: Option<LiteralType> = if let Ok(value) = v.parse::<u32>() {
        Some(LiteralType::Integer(value))
    } else if let Ok(value) = v.parse::<f64>() {
        Some(LiteralType::Float(value))
    } else {
        Some(LiteralType::String(v.to_string()))
    };
    result
}

fn get_literal_data(literal: &proc_macro2::Literal) -> Option<MyArgData> {
    let mut result: Option<MyArgData> = None;
    if let Some(r) = get_literal_type(&literal.to_string()) {
        match r {
            LiteralType::Integer(value) => {
                result = Some(MyArgData::Int(value));
            },
            LiteralType::Float(value) => {
                result = Some(MyArgData::Float(value));
            },
            LiteralType::String(value) => {
                let str = value.to_string();
                let value2 = &str[1..str.len() - 1];
                result = Some(MyArgData::String(value2.to_string()));
            },
        }
    }
    result
}

fn get_group_data(group: &proc_macro2::Group) -> Option<MyArgData> {
    let mut data: Option<MyArgData> = None;

    match group.delimiter() {
        proc_macro2::Delimiter::Parenthesis => { // Tuple
            
        },
        proc_macro2::Delimiter::Brace => {
            
        },
        proc_macro2::Delimiter::Bracket => { // Vec
            let iter = group.stream().into_iter();
            let mut temp_vec_int: Vec<u32> = Vec::new();
            let mut temp_vec_float: Vec<f64> = Vec::new();
            let mut temp_vec_string: Vec<String> = Vec::new();
            for item in iter {
                match item {
                    proc_macro2::TokenTree::Group(v) => {
                        data = get_group_data(&v);
                    },
                    proc_macro2::TokenTree::Ident(_) => {

                    },
                    proc_macro2::TokenTree::Punct(_) => {

                    },
                    proc_macro2::TokenTree::Literal(v) => {
                        let data = get_literal_data(&v);
                        if let Some(r) = data {
                            match r {
                                MyArgData::String(o) => temp_vec_string.push(o),
                                MyArgData::Int(o) => temp_vec_int.push(o),
                                MyArgData::Float(o) => temp_vec_float.push(o),
                                _ => {},
                            }
                        }
                    },
                }
            }
            if temp_vec_int.len() > 0 {
                data = Some(MyArgData::VecInteger(temp_vec_int));
            }
            if temp_vec_float.len() > 0 {
                data = Some(MyArgData::VecFloat(temp_vec_float));
            }
            if temp_vec_string.len() > 0 {
                data = Some(MyArgData::VecString(temp_vec_string));
            }
        },
        proc_macro2::Delimiter::None => {
            
        },
    }

    data
}


fn get_string_from_token_tree(tree: Option<&proc_macro2::TokenTree>) -> Option<String> {
    let mut result: Option<String> = None;
    if let Some(v) = tree {
        if let proc_macro2::TokenTree::Literal(literal) = v {
            if let Some(data) = get_literal_data(&literal) {
                if let MyArgData::String(p) = data {
                    result = Some(p);
                }
            }
        }
    }
    result
}

fn get_vec_integer_from_token_tree(tree: Option<&proc_macro2::TokenTree>) -> Option<Vec<u32>> {
    let mut result: Option<Vec<u32>> = None;
    if let Some(v) = tree {
        if let proc_macro2::TokenTree::Group(group) = v {
            if let Some(data) = get_group_data(group) {
                if let MyArgData::VecInteger(p) = data {
                    result = Some(p);
                }
            }
        }
    }
    result
}

fn get_named_parsing_real_token_stream(token_stream: &proc_macro2::TokenStream) -> Option<proc_macro2::TokenStream> {
    let mut real_token_stream: Option<proc_macro2::TokenStream> = None;
    let token_stream_collect_1: Vec<proc_macro2::TokenTree> = token_stream.clone().into_iter().collect();
    for item1 in token_stream_collect_1 {
        if let proc_macro2::TokenTree::Group(v1) = item1 {
            let token_stream_collect_2: Vec<proc_macro2::TokenTree> = v1.stream().into_iter().collect();
            for item2 in token_stream_collect_2 {
                if let proc_macro2::TokenTree::Group(v2) = item2 {
                    real_token_stream = Some(v2.stream());
                    break;
                }
            }
            break;
        }
    }
    real_token_stream
}