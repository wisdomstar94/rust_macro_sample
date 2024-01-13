extern crate proc_macro;

use std::ops::Deref;

use proc_macro::{TokenStream, TokenTree, Literal, Group};
use quote::quote;
use syn::{parse_macro_input, ItemFn, FnArg, Pat, Type, PathSegment};

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
struct NamedArguments {
    name: Option<String>,
    scores: Option<Vec<u32>>,
}

impl NamedArguments {
    pub fn new_and_parse(token: &TokenStream) -> Self {
        let mut obj = NamedArguments {
            name: None,
            scores: None,
        };
        let token_collect: Vec<TokenTree> = token.clone().into_iter().collect();
        let mut i = 0;
        for item in &token_collect {
            match item {
                TokenTree::Ident(v) => { // writed parameter name
                    match v.to_string().as_str() {
                        "name" => obj.name = get_string_from_token_tree(token_collect.get(i + 2)),
                        "scores" => obj.scores = get_vec_integer_from_token_tree(token_collect.get(i + 2)),
                        _ => {}
                    }
                },
                TokenTree::Group(_) => {
                    
                },
                TokenTree::Punct(_) => {
                    
                },
                TokenTree::Literal(_) => {
                    
                },
            }
            i = i + 1;
        }
        obj
    }
}

#[proc_macro_attribute]
pub fn my_custom_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Non-Named Arguments Parsing
    let non_named_arguments_vec = parse_attr_non_named(&attr);
    dbg!(non_named_arguments_vec);

    // Named Arguments Parsing
    let named_arguments = NamedArguments::new_and_parse(&attr);
    dbg!(named_arguments);

    let function = parse_macro_input!(item as ItemFn);
    let original_function_name = &function.sig.ident;
    let asyncness = &function.sig.asyncness;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let block = &function.block;
    let generics = &function.sig.generics;
    let vis = &function.vis;
    let constness = &function.sig.constness;
    let unsafety = &function.sig.unsafety;
    let abi = &function.sig.abi;

    // 인자 자체에 대한 커스텀이 필요한 경우..
    let mut new_args = vec![];
    for org_arg in inputs.iter() {
        match org_arg {
            FnArg::Receiver(_) => {
                // dbg!(x);
            },
            FnArg::Typed(k) => {
                let mut arg_name: Option<String> = None;
                if let Pat::Ident(ii) = k.pat.deref() {
                    arg_name = Some(ii.ident.to_string());
                }
                let mut arg_type: Option<String> = None;
                match k.ty.deref() {
                    Type::Path(o) => {
                        if let Some(PathSegment { ident: p, arguments: _ }) = o.path.segments.first() {
                            arg_type = Some(p.to_string());
                        }
                    },
                    Type::Reference(b) => {
                        if let Type::Path(aa) = b.elem.deref() {
                            if let Some(PathSegment { ident: p, arguments: _ }) = aa.path.segments.first() {
                                let mut temp = String::from("&");
                                temp.push_str(p.to_string().as_str());
                                arg_type = Some(temp);
                            }
                        }
                    },
                    _ => {

                    },
                }
                
                if let (
                    Some(a_name), 
                    Some(a_type)
                ) = (
                    arg_name, 
                    arg_type
                ) {
                    new_args.push(quote!(#a_name: #a_type));
                }
            },
        }
    }

    let expanded = quote! {
        #vis #asyncness #constness #unsafety #abi fn #original_function_name #generics (#inputs) #output {
            println!("Calling function: {}", stringify!(#original_function_name));
            #block
        }
    };

    TokenStream::from(expanded)
}

fn parse_attr_non_named(token: &TokenStream) -> Vec<MyArgData> {
    let mut sequential_args: Vec<MyArgData> = Vec::new();

    for item in token.clone().into_iter() {
        match item {
            TokenTree::Ident(v) => {
                sequential_args.push(MyArgData::RawStr(v.to_string()));
            },
            TokenTree::Group(v) => {
                if let Some(data) = get_group_data(&v) {
                    sequential_args.push(data);
                }
            },
            TokenTree::Punct(_) => {
                // ex) ,
            },
            TokenTree::Literal(v) => { 
                if let Some(data) = get_literal_data(&v) {
                    sequential_args.push(data);
                }
            },
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

fn get_literal_data(literal: &Literal) -> Option<MyArgData> {
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

fn get_group_data(group: &Group) -> Option<MyArgData> {
    let mut data: Option<MyArgData> = None;

    match group.delimiter() {
        proc_macro::Delimiter::Parenthesis => { // Tuple
            
        },
        proc_macro::Delimiter::Brace => {
            
        },
        proc_macro::Delimiter::Bracket => { // Vec
            let iter = group.stream().into_iter();
            let mut temp_vec_int: Vec<u32> = Vec::new();
            let mut temp_vec_float: Vec<f64> = Vec::new();
            let mut temp_vec_string: Vec<String> = Vec::new();
            for item in iter {
                match item {
                    TokenTree::Group(v) => {
                        data = get_group_data(&v);
                    },
                    TokenTree::Ident(_) => {

                    },
                    TokenTree::Punct(_) => {

                    },
                    TokenTree::Literal(v) => {
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
        proc_macro::Delimiter::None => {
            
        },
    }

    data
}

fn get_string_from_token_tree(tree: Option<&TokenTree>) -> Option<String> {
    let mut result: Option<String> = None;
    if let Some(v) = tree {
        if let TokenTree::Literal(literal) = v {
            if let Some(data) = get_literal_data(&literal) {
                if let MyArgData::String(p) = data {
                    result = Some(p);
                }
            }
        }
    }
    result
}

fn get_vec_integer_from_token_tree(tree: Option<&TokenTree>) -> Option<Vec<u32>> {
    let mut result: Option<Vec<u32>> = None;
    if let Some(v) = tree {
        if let TokenTree::Group(group) = v {
            if let Some(data) = get_group_data(group) {
                if let MyArgData::VecInteger(p) = data {
                    result = Some(p);
                }
            }
        }
    }
    result
}