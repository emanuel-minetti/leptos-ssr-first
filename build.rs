use std::path::PathBuf;
use leptos_i18n_build::TranslationsInfos;

fn main() {
    // trigger recompilation for sqlx
    println!("cargo:rerun-if-changed=migrations");

    // trigger recompilation for leptos-i18n
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // where to generate the translations
    let i18n_mod_directory = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("i18n");

    let translations_infos = TranslationsInfos::parse(Default::default()).unwrap();

    // emit "cargo:rerun-if-changed" for every translation file
    translations_infos.rerun_if_locales_changed();

    // codegen
    translations_infos
        .generate_i18n_module(i18n_mod_directory)
        .unwrap();
}