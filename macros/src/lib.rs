use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Literal;
use std::path::{PathBuf, Path};
use std::fs::read;
use std::env;

#[proc_macro]
pub fn include_mips(input: TokenStream) -> TokenStream {
    let tokens: Vec<_> = input.into_iter().collect();

    let path = match tokens.as_slice() {
        [TokenTree::Literal(lit)] => unwrap_string_literal(lit),
        _ => panic!("This macro only accepts a single, non-empty string argument"),
    };

    let base_path = resolve_path(&path);

    let mut layer: usize = 0;
    let mut mipmap_layers = vec![];

    loop {
        let next_path = base_path.join(&format!("mip_{}.png", layer));

        if !next_path.exists() {
            if layer == 0 {
                panic!("Image {} not found. Have you perhaps forgotten to add a build.rs step?", path)
            }
            break;
        }

        let data = read(next_path).unwrap();
        let literal = Literal::byte_string(&data);
        mipmap_layers.push(quote::quote! {
            mipmap::MipmapLayer::new(#layer, #literal)
        });

        layer += 1;
    }

    (quote::quote! {
        mipmap::Mipmap::new(&[ #(#mipmap_layers),* ])
    }).into()
}

fn unwrap_string_literal(lit: &proc_macro::Literal) -> String {
    let mut repr = lit.to_string();
    if !repr.starts_with('"') || !repr.ends_with('"') {
        panic!("This macro only accepts a single, non-empty string argument")
    }

    repr.remove(0);
    repr.pop();

    repr
}


fn resolve_path(
    raw: &str,
) -> PathBuf {
    Path::new(&env::var("OUT_DIR").unwrap()).join(raw)
}