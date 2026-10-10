use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Default)]
struct TranslationNode {
    key: Option<String>,
    children: BTreeMap<String, TranslationNode>,
}

fn main() {
    println!("cargo:rerun-if-changed=../lunaria-assets/assets/locales");

    let directory =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../lunaria-assets/assets/locales");

    let locales = rust_i18n::try_load_locales(directory.to_str().unwrap(), |_| false, true)
        .expect(" Failed to load translations");

    let mut root = TranslationNode::default();

    for key in locales.values().flat_map(|item| item.keys()) {
        let mut node = &mut root;

        for part in key.split('.') {
            node = node.children.entry(part.to_string()).or_default();
        }

        node.key = Some(key.clone());
    }

    let mut code = String::new();
    render(&root, &mut code);

    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("i18n_keys.rs");

    fs::write(output, code).expect("Failed to write translation keys");
}

fn render(node: &TranslationNode, code: &mut String) {
    for (name, child) in &node.children {
        if let Some(key) = &child.key {
            writeln!(
                code,
                "pub const {}: &str = {key:?};",
                name.to_ascii_uppercase()
            )
            .unwrap();
        }

        if !child.children.is_empty() {
            writeln!(code, "pub mod r#{name} {{").unwrap();
            render(child, code);
            writeln!(code, "}}").unwrap();
        }
    }
}
