fn main() {
    let dir = std::path::PathBuf::from("src");
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile("tree-sitter-celestia-property-list");
}
