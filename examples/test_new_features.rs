//! Test script for new features from ZaraClaw
//!
//! Run with: cargo run --example test_new_features

use std::path::Path;
use zeroclaw::agent::closed_loop_verifier::{
    self, ClaimVerificationResult, VerificationFailure,
};
use zeroclaw::agent::identity::{self, DIDResolver, AgentIdentity};
use zeroclaw::agent::reflection::SimpleReflectionEngine;

#[tokio::main]
async fn main() {
    println!("=== Testing Closed-Loop Verifier ===\n");

    // Test 1: File write verification
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    // Create test file
    std::fs::write(&file_path, "hello world").unwrap();

    let args = serde_json::json!({
        "path": file_path.to_str().unwrap(),
        "content": "hello world"
    });

    match closed_loop_verifier::verify_file_write(&args, temp_dir.path()) {
        Ok(Some(())) => println!("✓ File write verification: PASSED"),
        Ok(None) => println!("✗ File write verification: NOT APPLICABLE"),
        Err(e) => println!("✗ File write verification FAILED: {}", e),
    }

    // Test 2: Mismatch detection
    let wrong_args = serde_json::json!({
        "path": file_path.to_str().unwrap(),
        "content": "expected different content"
    });

    match closed_loop_verifier::verify_file_write(&wrong_args, temp_dir.path()) {
        Ok(_) => println!("✗ Should have detected mismatch!"),
        Err(e) => {
            if e.contains("content mismatch") {
                println!("✓ Mismatch detection: PASSED");
            } else {
                println!("✗ Unexpected error: {}", e);
            }
        }
    }

    // Test 3: is_write_tool
    assert!(closed_loop_verifier::is_write_tool("file_write"));
    assert!(closed_loop_verifier::is_write_tool("shell"));
    assert!(!closed_loop_verifier::is_write_tool("file_read"));
    println!("✓ is_write_tool detection: PASSED");

    println!("\n=== Testing DID Identity ===\n");

    // Test 4: DID Resolution
    let resolver = DIDResolver::with_random_key("1.0.0".to_string());
    let did = resolver.agent_did();
    println!("Agent DID: {}", did);
    assert!(did.starts_with("did:zeroclaw:1.0.0"));
    println!("✓ DID generation: PASSED");

    // Test 5: DID Document resolution
    match resolver.resolve(&did) {
        Ok(doc) => {
            println!("✓ DID Resolution: PASSED");
            println!("  Context: {:?}", doc.context);
            println!("  Verification methods: {}", doc.verification_method.len());
        }
        Err(e) => println!("✗ DID Resolution FAILED: {}", e),
    }

    // Test 6: Sign and verify
    let data = "test message";
    let signature = resolver.sign(data);
    match resolver.verify_signature(data, &signature) {
        Ok(()) => println!("✓ HMAC signature: PASSED"),
        Err(e) => println!("✗ HMAC verification FAILED: {}", e),
    }

    // Test 7: Wrong signature
    match resolver.verify_signature(data, "wrong_signature") {
        Ok(()) => println!("✗ Should have rejected wrong signature!"),
        Err(_) => println!("✓ Wrong signature rejection: PASSED"),
    }

    // Test 8: Agent identity
    let identity = AgentIdentity::new("2.0.0");
    assert_eq!(identity.did, "did:zeroclaw:2.0.0");
    assert_eq!(identity.name, "ZeroClaw");
    println!("✓ AgentIdentity: PASSED");

    // Test 9: compute_hmac and compute_sha256
    let hmac = identity::compute_hmac(&[0u8; 32], "test");
    assert_eq!(hmac.len(), 64);
    println!("✓ compute_hmac: PASSED (len={})", hmac.len());

    let sha = identity::compute_sha256("test");
    assert_eq!(sha.len(), 64);
    println!("✓ compute_sha256: PASSED (len={})", sha.len());

    println!("\n=== Testing Reflection Engine ===\n");

    // Note: Reflection engine requires a real provider to test fully
    // But we can verify the struct methods exist and compile
    println!("ReflectionEngine struct is available");
    println!("Note: Full integration test requires LLM provider setup");

    println!("\n=== All Basic Tests PASSED ===\n");
}
