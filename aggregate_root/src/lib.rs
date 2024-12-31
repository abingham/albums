use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(AggregateRoot)]
pub fn aggregate_root_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_aggregate_root(&ast)
}

fn impl_aggregate_root(ast: &syn::DeriveInput) -> TokenStream {
    // Implements Entity trait for a struct.
    // The struct must contain a field named `entity_attrs` of type `EntityAttrs`.
    let name = &ast.ident;
    let gen = quote! {
        impl Entity for #name {
            fn id(&self) -> UniqueId {
                self.entity_attrs.id
            }

            fn instance_id(&self) -> InstanceId {
                self.entity_attrs.instance_id
            }

            fn version(&self) -> u64 {
                self.entity_attrs.version
            }

            fn inc_version(&mut self) {
                self.entity_attrs.version += 1;
            }

            fn discarded(&self) -> bool {
                self.entity_attrs.discarded
            }
        }
    };
    gen.into()
}
