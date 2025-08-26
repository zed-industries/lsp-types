use lsp_types::*;
use serde_json;
use url::Url;

#[test]
fn test_uri_with_brackets_serialization() {
    // Create a URI with brackets like Fresh framework uses: [slug].tsx
    let file_path = "file:///Users/test/project/routes/blog/[slug].tsx";
    let url = Url::parse(file_path).expect("Should parse URL with brackets");
    
    // Test serialization of TextDocumentIdentifier
    let doc_id = TextDocumentIdentifier::new(url.clone());
    let serialized = serde_json::to_string(&doc_id).expect("Should serialize");
    println!("Serialized TextDocumentIdentifier: {}", serialized);
    
    // Test that the serialized JSON contains properly encoded brackets
    assert!(serialized.contains("%5B"), "Serialized JSON should contain encoded opening bracket");
    assert!(serialized.contains("%5D"), "Serialized JSON should contain encoded closing bracket");
    assert!(!serialized.contains("[slug]"), "Serialized JSON should not contain unencoded brackets");
    
    // Test deserialization - should handle encoded URLs correctly
    let deserialized: TextDocumentIdentifier = serde_json::from_str(&serialized).expect("Should deserialize");
    // The deserialized URL should now contain the encoded form
    assert_eq!(deserialized.uri.path(), "/Users/test/project/routes/blog/%5Bslug%5D.tsx", 
               "Deserialized URL should maintain encoded brackets");
    
    // Test Location with brackets
    let location = Location::new(url.clone(), Range::new(Position::new(0, 0), Position::new(0, 10)));
    let location_serialized = serde_json::to_string(&location).expect("Should serialize Location");
    println!("Serialized Location: {}", location_serialized);
    
    let location_deserialized: Location = serde_json::from_str(&location_serialized).expect("Should deserialize Location");
    // Check that Location serialization also contains encoded brackets
    assert!(location_serialized.contains("%5B"), "Location serialization should contain encoded brackets");
    assert_eq!(location.range, location_deserialized.range, "Ranges should match");
}

#[test] 
fn test_workspace_edit_with_bracket_uris() {
    use std::collections::HashMap;
    
    // Test WorkspaceEdit which uses the custom url_map serializer
    let file_path = "file:///Users/test/project/routes/blog/[slug].tsx";
    let url = Url::parse(file_path).expect("Should parse URL with brackets");
    
    let mut changes = HashMap::new();
    changes.insert(url.clone(), vec![TextEdit::new(
        Range::new(Position::new(0, 0), Position::new(0, 5)),
        "Hello".to_string()
    )]);
    
    let workspace_edit = WorkspaceEdit::new(changes);
    let serialized = serde_json::to_string(&workspace_edit).expect("Should serialize WorkspaceEdit");
    println!("Serialized WorkspaceEdit: {}", serialized);
    
    // Try to deserialize back
    let deserialized: WorkspaceEdit = serde_json::from_str(&serialized).expect("Should deserialize WorkspaceEdit");
    
    // Check that the URL with brackets is preserved (it should be encoded in the serialized form)
    if let Some(changes) = deserialized.changes {
        // The deserialized URL should be the encoded version
        let encoded_url = Url::parse("file:///Users/test/project/routes/blog/%5Bslug%5D.tsx").expect("Should parse encoded URL");
        assert!(changes.contains_key(&encoded_url), "Should contain encoded URL key");
    } else {
        panic!("WorkspaceEdit should have changes");
    }
}

#[test]
fn test_url_serialization_roundtrip() {
    let test_cases = vec![
        "file:///Users/test/[slug].tsx",
        "file:///Users/test/blog/[id]/[slug].tsx",
        "file:///Users/test/[[...slug]].tsx",
        "file:///Users/test/[category]/[...slug].tsx",
    ];
    
    for file_path in test_cases {
        println!("Testing: {}", file_path);
        let url = Url::parse(file_path).expect(&format!("Should parse URL: {}", file_path));
        
        // Direct URL serialization
        let url_serialized = serde_json::to_string(&url).expect("Should serialize URL");
        let url_deserialized: Url = serde_json::from_str(&url_serialized).expect("Should deserialize URL");
        
        println!("Original: {}", url);
        println!("Serialized: {}", url_serialized);
        println!("Deserialized: {}", url_deserialized);
        println!("---");
        
        assert_eq!(url, url_deserialized);
    }
}