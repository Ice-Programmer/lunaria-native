use proc_macro::TokenStream;
use quote::quote;
use syn::ItemStruct;

pub(super) fn expand(_args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let struct_info: ItemStruct = syn::parse(item)?;
    // 获取 struct 名称
    let name = &struct_info.ident;


    let quote = quote! {
        #struct_info

        impl<C> ::lunaria_di::BuildService<C> for #name
        where
            C: ::lunaria_di::RepositoryResolver,
        {
            fn build(cx: &C) -> ::anyhow::Result<Self> {
                let repository = cx.current_repository()?;
                Ok(Self::new(repository))
            }
        }
    };

    Ok(quote.into())
}