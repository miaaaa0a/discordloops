fn main() {
    embed_resource::compile("app-icon.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
