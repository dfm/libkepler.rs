use itertools::intersperse;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::str::FromStr;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, token::Comma, Ident, LitInt};

/// Compile time generation of a Householder iteration for Kepler's equation
#[proc_macro]
pub fn kepler_householder_step(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as HouseholderArgs);
    let numeric_type = &input.numeric_type;

    // We require that the order be at least 1 because anything lower is
    // undefined
    let order = input.order;
    if order < 1 {
        return quote::quote_spanned!(
            input.order_token.span() =>
            compile_error!("Householder order must be at least 1")
        )
        .into();
    }

    // The first ingredient we need is each derivative of Kepler's equation up
    // to the target order
    let function_derivatives = (0..order + 1)
        .map(|n| format_function_derivative(n, numeric_type))
        .collect::<TokenStream2>();

    // Next, each order update is constructed in sequence
    let updates = (1..order + 1)
        .map(|n| format_update(n, numeric_type))
        .collect::<TokenStream2>();

    // Finally, we return the last update from above
    let ret = TokenStream2::from_str(&format!("d{}", order)).unwrap();

    quote!({
        |ecc: #numeric_type, mean_anom: #numeric_type, ecc_anom: #numeric_type| -> #numeric_type {
            use num_traits::One;
            let ecc_sin = ecc * ecc_anom.sin();
            let ecc_cos = ecc * ecc_anom.cos();
            #function_derivatives
            #updates
            #ret
        }
    })
    .into()
}

struct HouseholderArgs {
    order_token: LitInt,
    order: usize,
    numeric_type: Ident,
}

impl Parse for HouseholderArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let order_token = input.parse::<LitInt>()?;
        let order = order_token.base10_parse()?;
        input.parse::<Comma>()?;
        let numeric_type = input.parse()?;
        Ok(HouseholderArgs {
            order_token,
            order,
            numeric_type,
        })
    }
}

fn format_function_derivative(order: usize, numeric_type: &Ident) -> TokenStream2 {
    let var = TokenStream2::from_str(&format!("f{}", order)).unwrap();
    match order {
        0 => quote!(let f0 = ecc_anom - ecc_sin - mean_anom;),
        1 => quote!(let f1 = #numeric_type::one() - ecc_cos;),
        2 => quote!(let f2 = ecc_sin;),
        3 => quote!(let f3 = ecc_cos;),
        _ => {
            if order % 2 == 0 {
                if order % 4 < 2 {
                    quote!(let #var = -ecc_sin;)
                } else {
                    quote!(let #var = ecc_sin;)
                }
            } else {
                if order % 4 < 2 {
                    quote!(let #var = -ecc_cos;)
                } else {
                    quote!(let #var = ecc_cos;)
                }
            }
        }
    }
}

fn format_update(order: usize, numeric_type: &Ident) -> TokenStream2 {
    let prev = TokenStream2::from_str(&format!("d{}", order - 1)).unwrap();
    let var = TokenStream2::from_str(&format!("d{}", order)).unwrap();
    let args = intersperse(
        (1..=order).map(|n| {
            let f = TokenStream2::from_str(&format!("f{}", n)).unwrap();
            if n <= 1 {
                f
            } else {
                let factor = factorial(n);
                quote!(#f / (#factor as #numeric_type))
            }
        }),
        quote!(,),
    )
    .collect::<TokenStream2>();
    quote!(let #var = -f0 / (libkepler_householder::horner!(#prev, #args));)
}

/// Compute the factorial of an integer at compile time
fn factorial(n: usize) -> usize {
    (1..=n).product()
}
