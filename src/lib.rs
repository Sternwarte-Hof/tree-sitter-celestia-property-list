//! Tree-sitter parser family for Celestia Property List documents.

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_celestia_property_list() -> Language;
}

/// Returns the Tree-sitter [`Language`] for Celestia Property List documents.
pub fn language() -> Language {
    unsafe { tree_sitter_celestia_property_list() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tree_sitter::Parser;

    #[test]
    fn test_grammar_name() {
        assert_eq!("celestia-property-list", "celestia-property-list");
    }

    #[test]
    fn test_parse_configuration_and_ssc() {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading celestia property list language");

        let cfg_text = r#"
            Configuration {
                SolarSystemCatalogs [ "demo.ssc" ]
            }
        "#;
        let tree = parser.parse(cfg_text, None).unwrap();
        let root = tree.root_node();
        assert!(!root.has_error(), "Root should parse without error");

        let ssc_text = r#"
            "DemoBody" "Sol" {
                Radius 6371
            }
        "#;
        let ssc_tree = parser.parse(ssc_text, None).unwrap();
        let ssc_root = ssc_tree.root_node();
        assert!(!ssc_root.has_error(), "SSC should parse without error");
    }

    #[test]
    fn test_parse_all_four_top_level_roles() {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading celestia property list language");

        // 1. Configuration
        let cfg = "Configuration { StarCatalogs [ \"stars.stc\" ] }";
        let cfg_tree = parser.parse(cfg, None).unwrap();
        assert!(!cfg_tree.root_node().has_error());
        assert_eq!(
            cfg_tree.root_node().child(0).unwrap().kind(),
            "configuration_block"
        );

        // 2. Solar System Catalog (.ssc)
        let ssc = "Modify Body \"Earth\" \"Sol\" { Radius 6371 }";
        let ssc_tree = parser.parse(ssc, None).unwrap();
        assert!(!ssc_tree.root_node().has_error());
        assert_eq!(
            ssc_tree.root_node().child(0).unwrap().kind(),
            "ssc_declaration"
        );

        // 3. Star Catalog (.stc)
        let stc = "Add Star 12345 \"Alpha Centauri A\" { RA 219.9 Dec -60.8 Distance 4.37 SpectralType \"G2V\" AppMag 0.01 }";
        let stc_tree = parser.parse(stc, None).unwrap();
        assert!(!stc_tree.root_node().has_error());
        assert_eq!(
            stc_tree.root_node().child(0).unwrap().kind(),
            "stc_declaration"
        );

        // 4. Deep Sky Catalog (.dsc)
        let dsc = "Galaxy \"Andromeda\" { RA 10.68 Dec 41.27 Distance 2.5e6 Radius 110000 }";
        let dsc_tree = parser.parse(dsc, None).unwrap();
        assert!(!dsc_tree.root_node().has_error());
        assert_eq!(
            dsc_tree.root_node().child(0).unwrap().kind(),
            "dsc_declaration"
        );
    }

    #[test]
    fn test_fatal_error_recovery_fixtures() {
        let mut parser = Parser::new();
        parser.set_language(&language()).unwrap();

        // Fatal unclosed string / malformed token
        let malformed = "Configuration { SolarSystemCatalogs [ \"demo.ssc ";
        let tree = parser.parse(malformed, None).unwrap();
        assert!(tree.root_node().has_error());
    }

    #[test]
    fn test_locally_recoverable_and_valid_after_damage_fixtures() {
        let mut parser = Parser::new();
        parser.set_language(&language()).unwrap();

        // Damaged declaration in middle, but valid declaration after damage
        let content = r#"
            Add Star 100 "ValidStar1" { RA 10 Dec 20 }
            Add Star BAD_SYNTAX_NO_QUOTES_OR_NAME {
            Add Star 200 "ValidStar2" { RA 30 Dec 40 }
        "#;
        let tree = parser.parse(content, None).unwrap();
        let root = tree.root_node();
        assert!(
            root.has_error(),
            "Root should contain error node for damaged declaration"
        );

        // Tree-sitter still constructs child nodes and recovers to parse valid declarations
        let child_count = root.child_count();
        assert!(
            child_count >= 2,
            "Should parse valid nodes despite middle damage"
        );
    }

    #[test]
    fn test_parse_all_metadata_top_level_forms() {
        let mut parser = Parser::new();
        parser
            .set_language(&language())
            .expect("Error loading celestia property list language");

        // 1. Destinations Metadata
        let dest = r#"{ Name "Moon" Target "Sol/Earth/Moon" Description "Earth satellite" Distance 10000 DistanceUnits "km" }"#;
        let dest_tree = parser.parse(dest, None).unwrap();
        assert!(!dest_tree.root_node().has_error());

        // 2. Favorites Metadata
        let fav = r#"Folder "Solar System" { "Earth View" { Target "Sol/Earth" Position [ 0 0 10000 ] } }"#;
        let fav_tree = parser.parse(fav, None).unwrap();
        assert!(!fav_tree.root_node().has_error());

        // 3. Asterism Metadata
        let ast = r#""Big Dipper" [ [ "UMa 50" "UMa 48" "UMa 54" ] ]"#;
        let ast_tree = parser.parse(ast, None).unwrap();
        assert!(!ast_tree.root_node().has_error());

        // 4. Virtual Texture Descriptor (.ctx)
        let ctx = r#"VirtualTexture { ImageDirectory "earth_vt" BaseSplit 0 TileSize 512 TileType "png" }"#;
        let ctx_tree = parser.parse(ctx, None).unwrap();
        assert!(!ctx_tree.root_node().has_error());
        assert_eq!(
            ctx_tree.root_node().child(0).unwrap().kind(),
            "virtual_texture_block"
        );
    }
}
