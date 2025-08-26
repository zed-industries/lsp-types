use lsp_types::*;
use serde_json;
use url::Url;

#[test]
fn test_bracket_percent_encoding_issue() {
    // This demonstrates the actual issue: brackets should be percent-encoded in URIs for LSP
    let file_path_with_brackets = "file:///Users/test/blog/[slug].tsx";
    
    // What the URL crate does by default
    let url = Url::parse(file_path_with_brackets).expect("Should parse URL");
    println!("Original URL: {}", url);
    println!("URL as_str(): {}", url.as_str());
    
    // When we serialize this with serde, it uses as_str() which doesn't percent-encode
    let doc_id = TextDocumentIdentifier::new(url.clone());
    let serialized = serde_json::to_string(&doc_id).expect("Should serialize");
    println!("Serialized (current): {}", serialized);
    
    // The issue is that LSP expects percent-encoded brackets
    // [ should become %5B and ] should become %5D
    let expected_encoded = r#"{"uri":"file:///Users/test/blog/%5Bslug%5D.tsx"}"#;
    println!("Expected (percent-encoded): {}", expected_encoded);
    
    // Show that we now have correct percent-encoding
    assert_eq!(serialized, expected_encoded, "Implementation should now percent-encode brackets");
}



#[test]
fn test_url_crate_behavior_with_brackets() {
    // Test how the url crate handles different ways of creating URLs with brackets
    
    // Method 1: Parse a string with unencoded brackets
    let url1 = Url::parse("file:///Users/test/[slug].tsx").expect("Should parse");
    println!("Method 1 - Parse unencoded: {}", url1.as_str());
    
    // Method 2: Build URL with encoded brackets
    let url2 = Url::parse("file:///Users/test/%5Bslug%5D.tsx").expect("Should parse");
    println!("Method 2 - Parse encoded: {}", url2.as_str());
    
    // Method 3: Use from_file_path if available
    #[cfg(any(unix, windows, target_os = "redox"))]
    {
        use std::path::Path;
        let path = Path::new("/Users/test/[slug].tsx");
        if let Ok(url3) = Url::from_file_path(path) {
            println!("Method 3 - from_file_path: {}", url3.as_str());
        }
    }
    
    // Show what happens during serialization
    let doc1 = TextDocumentIdentifier::new(url1);
    let doc2 = TextDocumentIdentifier::new(url2.clone());
    
    let ser1 = serde_json::to_string(&doc1).expect("Should serialize");
    let ser2 = serde_json::to_string(&doc2).expect("Should serialize");
    
    println!("Serialized unencoded source: {}", ser1);
    println!("Serialized encoded source: {}", ser2);
    
    // The key insight: url2.as_str() should give us the encoded version we want
    assert!(url2.as_str().contains("%5B") && url2.as_str().contains("%5D"));
}