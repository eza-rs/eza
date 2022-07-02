use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Lit, NestedMeta};

#[proc_macro_attribute]
pub fn eza_app(args: TokenStream, input: TokenStream) -> TokenStream {
    let id = parse_macro_input!(args as Vec<NestedMeta>);
    let user_main = parse_macro_input!(input as ItemFn);
    let fname = user_main.sig.ident.clone();

    if let [NestedMeta::Lit(Lit::Str(id))] = id.as_slice() {
        TokenStream::from(quote! {
            fn main() -> Result<(), eza::AppError> {
                let app = eza::NativeApp::new(#id)?;

                app.run({ #user_main #fname })
            }
        })
    } else {
        panic!("Expected the id of the application as argument");
    }
}
