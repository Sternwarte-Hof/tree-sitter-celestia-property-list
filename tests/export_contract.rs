#[test]
fn exported_parser_and_highlight_query_compile() {
    let language = tree_sitter_celestia_property_list::language();
    tree_sitter::Query::new(&language, include_str!("../queries/highlights.scm"))
        .expect("exported highlight query must compile against exported parser");
}
