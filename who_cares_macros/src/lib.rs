use proc_macro::TokenStream;

use quote::quote;
use syn::parse::Nothing;
use syn::{Expr, ItemFn, Stmt, parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn who_cares(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(attr as Nothing);

    let mut function = parse_macro_input!(item as ItemFn);
    function.sig.output = parse_quote!(-> ::who_cares::WhoCares<()>);
    append_who_cares_return(&mut function);

    quote!(#function).into()
}

fn append_who_cares_return(function: &mut ItemFn) {
    if let Some(last) = function.block.stmts.last_mut()
        && let Stmt::Expr(_, semi @ None) = last
    {
        *semi = Some(Default::default());
    }

    let who_cares_return: Expr = parse_quote!(::who_cares::WhoCares(()));
    function
        .block
        .stmts
        .push(Stmt::Expr(who_cares_return, None));
}
