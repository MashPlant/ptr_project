use proc_macro::TokenStream;
use syn::{parse, ItemStruct, Field};
use quote::quote;

#[proc_macro_attribute]
pub fn ptr_project(_attr: TokenStream, item: TokenStream) -> TokenStream {
  let s = parse::<ItemStruct>(item).expect("#[ptr_project] expect struct");
  let (vis, ident) = (&s.vis, &s.ident);
  let (impl_generics, ty_generics, where_clause) = s.generics.split_for_impl();
  let ptr_fields = s.fields.iter().map(|Field { vis, ident, ty, .. }|
    quote! { #vis #ident: *const #ty });
  let ptr_mut_fields = s.fields.iter().map(|Field { vis, ident, ty, .. }|
    quote! { #vis #ident: *mut #ty });
  let fields1 = s.fields.iter().map(|Field { ident, .. }| ident);
  let (fields2, fields3, fields4) = (fields1.clone(), fields1.clone(), fields1.clone());
  let res = quote! {
    #s

    #[doc(hidden)]
    #[allow(dead_code)]
    #[allow(non_upper_case_globals)]
    #[allow(single_use_lifetimes)]
    #[allow(clippy::used_underscore_binding)]
    const _: () = {
      #vis struct PtrProjection #ty_generics {
        #(#ptr_fields),*
      }
      #vis struct MutPtrProjection #ty_generics {
        #(#ptr_mut_fields),*
      }
      impl #impl_generics #ident #ty_generics #where_clause {
        #vis fn project(self: *const Self) -> PtrProjection #ty_generics {
          let Self { #(#fields1),* } = unsafe { &*self };
          PtrProjection { #(#fields2),* }
        }
        #vis fn project_mut(self: *mut Self) -> MutPtrProjection #ty_generics {
          let Self { #(#fields3),* } = unsafe { &mut *self };
          MutPtrProjection { #(#fields4),* }
        }
      }
    };
  };
  res.into()
}