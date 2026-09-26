use telar::i18n::CatalogModel;

use super::CONTENT;

fn load_catalog() -> CatalogModel {
    let es = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/es.toml"))
        .expect("reading locales/es.toml");
    let en = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/en.toml"))
        .expect("reading locales/en.toml");
    CatalogModel::from_sources(
        &[("es", &es, "locales/es.toml"), ("en", &en, "locales/en.toml")],
        Some("es"),
    )
    .expect("locales/es.toml and locales/en.toml must parse as valid catalogs")
}

#[test]
fn catalogs_share_the_same_key_set() {
    let catalog = load_catalog();
    for (key, per_locale) in catalog.entries.iter() {
        for locale in ["es", "en"] {
            assert!(
                per_locale.contains_key(locale),
                "key `{key}` is missing from the `{locale}` catalog"
            );
        }
    }
}

#[test]
fn content_model_keys_all_exist_in_the_catalog() {
    let catalog = load_catalog();
    for key in CONTENT.referenced_keys() {
        assert!(
            catalog.contains_key(key),
            "content module references catalog key `{key}`, which is not in locales/es.toml or locales/en.toml"
        );
    }
}
