fn main() {
    generate_lexicon_types();
}

fn generate_lexicon_types() {
    const LEXICONS_DIR: &str = "./lexicons";
    const LEXICONS_OUT_DIR: &str = "./src/api/xrpc/__lexicon";

    println!("cargo:rerun-if-changed=lexicons");

    if !std::fs::exists(LEXICONS_OUT_DIR).unwrap_or_default() {
        std::fs::create_dir(LEXICONS_OUT_DIR).unwrap();
    }
    atrium_codegen::genapi(
        LEXICONS_DIR,
        LEXICONS_OUT_DIR,
        &[
            ("com.atproto", None),
            ("chat.bsky", None),
            ("app.bsky", None),
            ("tools.ozone", None),
        ],
    )
    .unwrap_or_else(|err| panic!("{err}"));
}
