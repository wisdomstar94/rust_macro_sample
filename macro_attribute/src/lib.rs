extern crate proc_macro;

use std::ops::Deref;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Ident, LitStr, LitInt, Token, parse::{Parse, ParseStream}, FnArg, Pat, Type, PathSegment};

// struct MyCustomNamedAttributes {
//     list: Vec<syn::Ident>,
//     read: Vec<syn::Ident>,
// }

#[derive(Debug)]
struct MyCustomArgs {
    arg1: Result<Ident, syn::Error>,
    _comma1: Result<Token![,], syn::Error>,
    arg2: Result<LitStr, syn::Error>,
    _comma2: Result<Token![,], syn::Error>,
    arg3: Result<LitInt, syn::Error>,
}

#[derive(Debug)]
enum MyArgData {
    Str(String),
    Int(u32),
}

impl Parse for MyCustomArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(MyCustomArgs {
            arg1: input.parse(),
            _comma1: input.parse(),
            arg2: input.parse(),
            _comma2: input.parse(),
            arg3: input.parse(),
        })
    }
}

// #[derive(Debug)]
// struct MyCustomNamesArgs {
//     scores: Result<Vec<u32>, syn::Error>,
//     name: Result<String, syn::Error>,
// }

// impl Parse for MyCustomNamesArgs {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let arg_name: Ident = input.parse()?;
//         if arg_name != "name" {
//             // Same error as before when encountering an unsupported attribute
//             return Err(syn::Error::new_spanned(
//                 arg_name,
//                 "unsupported getter attribute, expected `name`",
//             ));
//         }

//         // Parse (and discard the span of) the `=` token
//         let _: Token![=] = input.parse()?;

//         // Parse the argument value
//         // let name = input.parse();

//         Ok(MyCustomNamesArgs { 
//             name: Ok(String::from("")),
//             scores: Ok(vec![1]),
//         })
//     }
// }

#[proc_macro_attribute]
pub fn my_custom_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Non-Named Arguments Parsing
    let mut sequential_args: Vec<MyArgData> = Vec::new();
    let clone_attr = attr.clone();
    let macro_args = parse_macro_input!(clone_attr as MyCustomArgs);
    if let Ok(r) = macro_args.arg1 {
        sequential_args.push(MyArgData::Str(r.to_string()));
    }
    if let Ok(r) = macro_args.arg2 {
        sequential_args.push(MyArgData::Str(r.value()));
    }
    if let Ok(r) = macro_args.arg3 {
        if let Ok(v) = r.base10_parse::<u32>() {
            sequential_args.push(MyArgData::Int(v));
        }
    }
    dbg!(sequential_args);

    // Named Arguments Parsing
    // let named_macro_args = parse_macro_input!(attr as MyCustomNamesArgs);
    // let a = Attribute::parse_args(attr.);
    // let named_macro_args = parse_macro_input!(attr as MyCustomNamesArgs);
    // dbg!(named_macro_args);

    let function = parse_macro_input!(item as ItemFn);
    let original_function_name = &function.sig.ident;
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
        #vis #constness #unsafety #abi fn #original_function_name #generics (#inputs) #output {
            println!("Calling function: {}", stringify!(#original_function_name));
            #block
        }
    };

    TokenStream::from(expanded)
}
