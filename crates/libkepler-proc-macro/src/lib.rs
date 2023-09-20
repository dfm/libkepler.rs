use itertools::intersperse;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::{
    parse_macro_input,
    token::{Comma, Not},
    LitInt, Path,
};

struct Arg {
    function: Path,
    bang_token: Option<Not>,
    evaluator: Path,
    number: usize,
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let function = input.parse()?;
        let bang_token = input.parse().unwrap_or(None);
        input.parse::<Comma>()?;
        let evaluator = input.parse()?;
        input.parse::<Comma>()?;
        let number = input.parse::<LitInt>()?.base10_parse()?;
        Ok(Arg {
            function,
            bang_token,
            evaluator,
            number,
        })
    }
}

#[proc_macro]
pub fn expand_to_order(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Arg);
    let function = &input.function;
    let bang_token = &input.bang_token;
    let evaluator = &input.evaluator;
    let collected: TokenStream2 = intersperse(
        (0..input.number).map(|number| quote::quote!(#evaluator::<#number>())),
        quote::quote!(,),
    )
    .collect();
    quote::quote!(#function #bang_token (#collected)).into()
}
