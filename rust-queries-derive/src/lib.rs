extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

/// Derive macro to generate Queryable implementations
/// 
/// # Example
/// 
/// ```ignore
/// #[derive(Queryable)]
/// struct Product {
///     id: u32,
///     name: String,
///     price: f64,
/// }
/// ```
#[proc_macro_derive(Queryable)]
pub fn derive_queryable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics rust_queries_core::Queryable<#name #ty_generics> for Vec<#name #ty_generics> #where_clause {
            fn query_iter(&self) -> Box<dyn Iterator<Item = &#name #ty_generics> + '_> {
                Box::new(self.iter())
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro to generate helper methods for query building
/// 
/// This macro generates convenience methods for common query patterns
/// 
/// # Example
/// 
/// ```ignore
/// #[derive(QueryBuilder)]
/// struct Product {
///     id: u32,
///     name: String,
///     price: f64,
/// }
/// ```
#[proc_macro_derive(QueryBuilder)]
pub fn derive_query_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Extract field names for documentation
    let fields = match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    fields.named.iter().map(|f| f.ident.as_ref().unwrap()).collect::<Vec<_>>()
                }
                _ => vec![],
            }
        }
        _ => vec![],
    };

    let field_docs = if !fields.is_empty() {
        let field_list = fields.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n///   ");
        format!("/// Available fields:\n///   {}", field_list)
    } else {
        String::new()
    };

    let expanded = quote! {
        impl #name {
            #[doc = #field_docs]
            /// Creates a new eager Query from a slice of items
            pub fn query(items: &[Self]) -> rust_queries_core::Query<Self> {
                rust_queries_core::Query::new(items)
            }

            #[doc = #field_docs]
            /// Creates a new lazy Query from a slice of items
            pub fn lazy_query(items: &[Self]) -> rust_queries_core::LazyQuery<Self, impl Iterator<Item = &Self>> {
                rust_queries_core::LazyQuery::new(items)
            }
        }
    };

    TokenStream::from(expanded)
}
