use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;

#[proc_macro]
pub fn not_working(input: TokenStream) -> TokenStream {
    let file_string = format!("{}/output_example.rs", env!("OUT_DIR"));
    let file_path = Path::new(&file_string);
    let _ = std::fs::write(&file_path, "const EXAMPLE: &str = \"blah\";");
    let file_out = file_path.to_string_lossy().to_string();

    quote! {
        include!(#file_out);
    }
    .into()
}
