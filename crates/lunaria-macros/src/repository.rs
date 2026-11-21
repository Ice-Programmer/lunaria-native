use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, ItemImpl, MetaNameValue};

pub fn expand(args: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let attr: MetaNameValue = syn::parse(args)?;
    let impl_info: ItemImpl = syn::parse(item)?;

    if !attr.path.is_ident("key") {
        return Err(Error::new_spanned(
            attr.path,
            "expected #[repository(key = ???)]",
        ));
    };

    let key = &attr.value;

    // 获取 impl 的 XXRepository
    let Some((None, interface, _)) = &impl_info.trait_ else {
        return Err(Error::new_spanned(
            impl_info,
            "[repository] must be placed on a trait impl",
        ));
    };

    // 取出 for 后面具体类型的 Repository
    let repository_type = &impl_info.self_ty;

    let expanded = quote! {
        #impl_info

        ::lunaria_di::submit! {
            ::lunaria_di::RepositoryFactory {
                key: #key,
                factory: {
                    const CREATE: fn(&::lunaria_di::Backend) -> ::anyhow::Result<::std::sync::Arc<dyn #interface>> = |backend| {
                        let backend = ::lunaria_di::backend_as(backend)?;
                        let repository: ::std::sync::Arc<#repository_type> = <#repository_type>::new(backend).into();

                        Ok(repository)
                    };

                    &CREATE
                }
            }
        }
    };

    Ok(expanded.into())
}
