use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use syn::{__private::{ToTokens, quote::format_ident}, parse_quote};

/// Makes the attached function record its execution time using the `record_data` function.
///
/// # Example
///
/// ```
/// use rust_profiler::profiled;
///
/// #[profiled(slow_function)]
/// fn slow_function(seconds: u64) {
///    std::thread::sleep(std::time::Duration::from_secs(seconds));
/// }
/// ```
#[proc_macro_attribute]
pub fn profiled(attribute_parameters: TokenStream, item: TokenStream) -> TokenStream {
    // The attribute should have one parameter.
    let function_identifier = syn::parse_macro_input!(attribute_parameters as syn::Ident);
    let function_identifier = function_identifier.to_string();
    let rust_profiler_crate =
        match crate_name("rust-profiler").expect("Failed to find rust-profiler crate") {
            FoundCrate::Itself => format_ident!("crate"),
            FoundCrate::Name(name) => format_ident!("{}", name),
        };
    // The item should be a function.
    let mut item = syn::parse_macro_input!(item as syn::ItemFn);
    let item_block = &item.block;
    let new_function_block = parse_quote! {
        {
            let start = ::std::time::Instant::now();
            // We have to put it in a closure so that early returns don't prevent the profiler from profiling.
            let result = (move || #item_block)();
            let elapsed = start.elapsed();
            #rust_profiler_crate::record_data(#function_identifier, elapsed);
            result
        }
    };
    item.block = Box::new(new_function_block);
    item.to_token_stream().into()
}
