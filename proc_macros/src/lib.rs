use proc_macro::TokenStream;
use syn::{__private::ToTokens, parse_quote};

#[proc_macro_attribute]
pub fn profiled(attribute_parameters: TokenStream, item: TokenStream) -> TokenStream {
    // The parameters should consist of nothing but an identifier.
    let attribute_parameter = syn::parse_macro_input!(attribute_parameters as syn::Ident);
    let function_identifier = attribute_parameter.to_string();
    // The item should be a function.
    let mut item = syn::parse_macro_input!(item as syn::ItemFn);
    let item_block = &item.block;
    let new_function_block = parse_quote! {
        {
            let start = ::std::time::Instant::now();
            // We have to put it in a closure so that early returns don't prevent the profiler from profiling.
            let result = (move || #item_block)();
            let elapsed = start.elapsed();
            crate::record_data(#function_identifier, elapsed);
            result
        }
    };
    item.block = Box::new(new_function_block);
    item.to_token_stream().into()
}
