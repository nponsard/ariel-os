use quote::format_ident;

const ARIEL_OS_CRATE_NAME: &str = "ariel-os";

/// Returns a [`struct@syn::Ident`] identifying the `ariel-os` dependency.
///
/// # Panics
///
/// - Panics when the `ariel-os` crate cannot be found as a dependency of the crate in which
///   this function is called.
/// - Panics if `ariel-os` is used as a dependency of itself.
pub fn ariel_os_crate() -> syn::Ident {
    ariel_os_crate_or_internal(None)
}

/// Returns a [`struct@syn::Ident`] identifying the `ariel-os` dependency, or an (internal)
/// fallback crate that provides the same relevant items.
///
/// # Panics
///
/// - Panics when neither crate can be found as a dependency of the crate in which
///   this function is called.
/// - Panics if either crate is used as a dependency of itself.
pub fn ariel_os_crate_or_internal(internal: Option<&'static str>) -> syn::Ident {
    find_crate(ARIEL_OS_CRATE_NAME)
        .or_else(|| find_crate(internal?))
        .unwrap_or_else(|| {
            panic!(
                "{ARIEL_OS_CRATE_NAME} should be present in `Cargo.toml`{}",
                internal
                    .map(|i| format!(" (internal crates may also use {i})"))
                    .unwrap_or_default()
            )
        })
}

/// Returns a [`struct@syn::Ident`] identifying the `name` dependency (or `None`).
///
/// # Panics
///
/// - Panics if `name` is used as a dependency of itself.
pub fn find_crate(name: &str) -> Option<syn::Ident> {
    if let Ok(crate_) = proc_macro_crate::crate_name(name) {
        match crate_ {
            proc_macro_crate::FoundCrate::Itself => {
                panic!("{name} cannot be used as a dependency of itself");
            }
            proc_macro_crate::FoundCrate::Name(crate_) => Some(format_ident!("{crate_}")),
        }
    } else {
        None
    }
}
