#![recursion_limit = "65536"]
#[macro_use]
extern crate quote;
extern crate syn;

use std::iter::zip;

use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Field, Token, TypePath};
// #[proc_macro_derive(Model)]
// pub fn derive_model(input: TokenStream) -> TokenStream {
//     let ast: syn::DeriveInput = syn::parse(input).unwrap();

//     let fields: Vec<syn::Field> = match ast.data {
//         syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
//             if fields.iter().any(|field| field.ident.is_none()) {
//                 panic!("#[derive(Model)] struct cannot have unnamed field");
//             }
//             fields.iter().cloned().collect()
//         }
//         _ => panic!("#[derive(Model)] can only be used with structs"),
//     };

//     let mut field_idents = vec![];
//     let mut field_names = vec![];

//     for field in fields {
//         let field_ident = field.ident.unwrap();
//         match field.ty {
//             syn::Type::Path(..) => {}
//             syn::Type::Array(..) => {}
//             _ => panic!(
//                 "Type `{:?}` of field `{:?}` is not supported",
//                 field.ty, field_ident
//             ),
//         };

//         field_names.push(field_ident.to_string());
//         field_idents.push(field_ident);
//     }

//     let struct_name = &ast.ident;
//     let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

//     let impl_ast = quote! {
//         impl #impl_generics yew_form::model::Model for #struct_name #ty_generics #where_clause { }

//         impl #impl_generics yew_form::model::FormValue for #struct_name #ty_generics #where_clause {
//             fn fields(&self, mut prefix: &::std::primitive::str, fields: &mut ::std::vec::Vec<::yew::virtual_dom::AttrValue>) {
//                 let field_prefix = if prefix.is_empty() {
//                     ::std::string::String::default()
//                 } else {
//                     ::std::format!("{}.", prefix)
//                 };

//                 #(
//                     let field_path = ::std::format!("{}{}", field_prefix, #field_names);
//                     self.#field_idents.fields(&field_path, fields);
//                 )*
//             }

//             fn value(&self, field_path: &::std::primitive::str) -> ::yew::virtual_dom::AttrValue {
//                 let (field_name, suffix) = yew_form::split_field_path(field_path);

//                 match field_name {
//                     #(
//                         #field_names => self.#field_idents.value(suffix),
//                     )*
//                     _ => ::std::panic!("Field {} does not exist in {}", field_path, ::std::stringify!(#struct_name))
//                 }
//             }

//             fn set_value(&mut self, field_path: &::std::primitive::str, value: &::std::primitive::str) -> ::std::result::Result<(), &'static ::std::primitive::str> {
//                 let (field_name, suffix) = yew_form::split_field_path(field_path);

//                 match field_name {
//                     #(
//                         #field_names => self.#field_idents.set_value(suffix, value),
//                     )*
//                     _ => ::std::panic!("Field {} does not exist in {}", field_path, ::std::stringify!(#struct_name))
//                 }
//             }

//         }
//     };

//     impl_ast.into()
// }
//
//

#[proc_macro_derive(Model, attributes(model))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let fields: Vec<syn::Field> = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                panic!("#[derive(Model)] struct cannot have unnamed field");
            }

            fields.iter().cloned().collect()
        }
        _ => panic!("#[derive(Model)] can only be used with structs"),
    };

    let mut field_idents = vec![];
    let mut field_names = vec![];
    let mut field_enum_variants = vec![];
    let mut field_enum_types = vec![];
    let mut field_models = vec![];

    for field in fields {
        let field_ident = field.ident.unwrap();
        let field_type = match field.ty {
            syn::Type::Path(..) => field.ty,
            //            syn::Type::Array(..) => {}
            _ => panic!(
                "Type `{:?}` of field `{:?}` is not supported",
                field.ty, field_ident
            ),
        };

        field_enum_variants.push(format_ident!(
            "{}",
            field_ident.to_string().to_pascal_case(),
        ));
        field_models.push(field.attrs.iter().any(|attr| attr.path.is_ident("model")));
        field_enum_types.push(field_type);
        field_names.push(field_ident.to_string());
        field_idents.push(field_ident);
    }

    let struct_name = &ast.ident;
    let enum_name = format_ident!("{}Fields", struct_name);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let visibility = ast.vis;

    // let mut impl_prefix_path = quote! {};

    // for (i, model) in field_models.iter().enumerate() {
    //     let field_enum_variant = &field_enum_variants[i];
    //     let field_enum_type = &field_enum_types[i];

    //     if *model {
    //         impl_prefix_path.extend(quote!{

    //         impl PrefixPath<<#field_enum_type as ModelTrait>::Path> for #enum_name {
    //             fn prefix(prefix: Self, path: <#field_enum_type as ModelTrait>::Path) -> Self {
    //                 match prefix {
    //                     Self::#field_enum_variant(root) if root == <<#field_enum_type as ModelTrait>::Path as IntoField>::root() => {
    //                         Self::#field_enum_variant(path)
    //                     },
    //                     _ => panic!(),
    //                 }
    //             }
    //         }

    //         })
    //     }
    // }

    let mut enum_scalar_names = vec![];
    let mut enum_scalar_variants = vec![];

    let mut enum_model_names = vec![];
    let mut enum_model_variants = vec![];
    let mut enum_model_types = vec![];
    let mut enum_model_idents = vec![];

    for (i, is_model) in field_models.into_iter().enumerate() {
        if is_model {
            enum_model_names.push(&field_names[i]);
            enum_model_variants.push(&field_enum_variants[i]);
            enum_model_types.push(&field_enum_types[i]);
            enum_model_idents.push(&field_idents[i]);
        } else {
            enum_scalar_names.push(&field_names[i]);
            enum_scalar_variants.push(&field_enum_variants[i]);
        }
    }
    // for ((is_model, variant), ty) in
    //     zip(zip(&field_models, &field_enum_variants), &field_enum_types)
    // {
    //     if *is_model {
    //         enum_variants.extend(quote! { #variant(<#ty as ModelTrait>::Path), })
    //     } else {
    //     }
    // }

    quote! {
        #[derive(Debug, Clone, Copy, ::std::hash::Hash, PartialEq, Eq)]
        #visibility enum #enum_name {
            Root,
            #(
                #enum_scalar_variants,
            )*
            #(
                #enum_model_variants(<#enum_model_types as ModelTrait>::Path),
            )*
        }


        impl IntoField for #enum_name {
            fn into_field(path: &'static str) -> Self {
                debug_assert!(!path.is_empty());
                match path.split_once('.') {
                    None => {
                        match path {
                            "" => Self::Root,
                            #(
                                #enum_scalar_names => Self::#enum_scalar_variants,
                                // #field_names => Self::#field_enum_variants((<#field_enum_types as ModelTrait>::Path::into_field(""))),
                            )*
                            _ => panic!(),
                        }
                    },
                    Some((field, suffix)) => match field {
                        #(
                            #enum_model_names => Self::#enum_model_variants(<#enum_model_types as ModelTrait>::Path::into_field(suffix)),
                        )*
                        _ => panic!(),
                    },
                }
            }

            fn root() -> Self {
                Self::Root
            }

            // fn prefix<T: IntoField>(self, path: T) -> Self {
            //     match self {
            //         #
            //         (
            //             // Self::Root => {
            //             //     path
            //             // }
            //             Self::#enum_model_variants(root @ <#enum_model_types as ModelTrait>::Path::Root) => {
            //                 Self::#enum_model_variants(root.replace_root(path))
            //             },
            //             Self::#enum_model_variants(model) => {
            //                 Self::#enum_model_variants(model.prefix(path))
            //             },
            //         )*
            //         // ParentFields::Child(ChildFields::Root) => ParentFields::Child(ChildFields::into_field(path)),
            //         // ParentFields::Child(c) => c.with_prefix(path),
            //         _ => panic!(),
            //     }
            // }

            // fn replace_root(self, path: Self) -> Self {
            //     path
            // }
        }

        // #(
        //     impl PrefixPath<<#field_enum_types as ModelTrait>::Path> for #fields_enum_name {
        //         fn prefix(prefix: Self, path: <#field_enum_types as ModelTrait>::Path) -> Self {
        //             match prefix {
        //                 Self::#field_enum_variants(root) if root == <<#field_enum_types as ModelTrait>::Path as IntoField>::root() => {
        //                     Self::#field_enum_variants(path)
        //                 },
        //                 _ => panic!(),
        //             }
        //         }
        //     }
        // )*

        impl #impl_generics yew_form::new_form::ModelTrait for #struct_name #ty_generics #where_clause {
            type Path = #enum_name;

            fn paths(&self) -> ::std::vec::Vec<Self::Path> {
                let mut paths = ::std::vec![#(
                    Self::Path::#enum_scalar_variants
                ),*];

                #(
                    paths.extend(self.#enum_model_idents.paths().into_iter().map(Self::Path::#enum_model_variants));
                )*

                paths
            }

            fn value(&self, path: Self::Path) -> ::yew::virtual_dom::AttrValue {
                todo!()
                // let (field_name, suffix) = yew_form::split_field_path(path);

                // match field_name {
                //     #(
                //         #field_names => self.#field_idents.value(suffix),
                //     )*
                //     _ => ::std::panic!("Field {} does not exist in {}", path, ::std::stringify!(#struct_name))
                // }
            }

            fn set_value(&mut self, path: Self::Path, value: &::std::primitive::str) -> ::std::result::Result<(), ::std::string::String> {
                todo!()
                // let (field_name, suffix) = yew_form::split_field_path(path);

                // match field_name {
                //     #(
                //         #field_names => self.#field_idents.set_value(suffix, value),
                //     )*
                //     _ => ::std::panic!("Field {} does not exist in {}", path, ::std::stringify!(#struct_name))
                // }
            }

        }
    }.into()
}

struct FieldInput {
    fields: Punctuated<Ident, Token![.]>,
}

impl Parse for FieldInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: input.parse_terminated(Ident::parse)?,
        })
    }
}

#[proc_macro]
pub fn field(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as FieldInput);

    let field_names: Vec<String> = input
        .fields
        .iter()
        .take(input.fields.len() - 1)
        .map(|i| i.to_string().to_pascal_case())
        .collect();
    let last_name: String = input.fields.last().unwrap().to_string().to_pascal_case();

    quote! {
        #(
            _::#field_names
        )*
        #last_name

    }
    .into()
}
