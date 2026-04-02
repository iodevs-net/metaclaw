//! Decentralized Identity (DID) for ZeroClaw agent.
//!
//! Implements a minimal DID method `did:zeroclaw:<version>` following
//! W3C DID Core 1.0. The DID document is generated dynamically from the
//! agent version — no distributed registry or blockchain required.

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// Compute HMAC-SHA256 over data with the given key.
pub fn compute_hmac(key: &[u8], data: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key size");
    mac.update(data.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

/// Compute SHA-256 hash of data.
pub fn compute_sha256(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}

/// Errors that can occur during DID operations.
#[derive(Debug, Clone)]
pub enum DIDError {
    /// The DID format is invalid.
    InvalidDID(String),
    /// The DID method is not supported.
    UnsupportedMethod(String),
    /// Verification failed.
    VerificationFailed(String),
    /// DID not found.
    NotFound(String),
}

impl std::fmt::Display for DIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DIDError::InvalidDID(d) => write!(f, "Invalid DID: {}", d),
            DIDError::UnsupportedMethod(m) => write!(f, "Unsupported DID method: {}", m),
            DIDError::VerificationFailed(e) => write!(f, "Verification failed: {}", e),
            DIDError::NotFound(d) => write!(f, "DID not found: {}", d),
        }
    }
}

impl std::error::Error for DIDError {}

/// A verification method entry in a DID Document.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationMethod {
    /// Unique identifier of this verification method.
    pub id: String,
    /// Type of verification method.
    #[serde(rename = "type")]
    pub verification_type: String,
    /// Controller of this verification method.
    pub controller: String,
}

/// A DID Document conforming to W3C DID Core 1.0.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DIDDocument {
    /// JSON-LD @context.
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    /// The DID that this document describes.
    pub id: String,
    /// Verification methods for this DID.
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,
    /// Authentication methods (can authenticate as the DID subject).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authentication: Vec<String>,
    /// Assertion methods (can make assertions on behalf of the DID subject).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Vec<String>,
}

/// DID Resolver for `did:zeroclaw:` method.
///
/// Generates DID Documents dynamically from agent version.
/// No distributed registry — this is a "static" DID method.
#[derive(Debug, Clone)]
pub struct DIDResolver {
    version: String,
    signing_key: Vec<u8>,
}

impl DIDResolver {
    /// Create a new resolver from version and HMAC signing key.
    ///
    /// The signing key must be 32 bytes (256 bits).
    pub fn new(version: String, signing_key: Vec<u8>) -> Self {
        Self { version, signing_key }
    }

    /// Create a new resolver with a random signing key.
    pub fn with_random_key(version: String) -> Self {
        let signing_key: Vec<u8> = (0..32).map(|_| rand::random()).collect();
        Self { version, signing_key }
    }

    /// Get the DID for this agent.
    pub fn agent_did(&self) -> String {
        format!("did:zeroclaw:{}", self.version)
    }

    /// Resolve a DID to its DID Document.
    ///
    /// Returns the DID Document if the DID is valid and supported.
    pub fn resolve(&self, did: &str) -> Result<DIDDocument, DIDError> {
        // Parse DID: did:zeroclaw:<version>
        let parsed = Self::parse_did(did)?;

        // Validate method
        if parsed.method != "zeroclaw" {
            return Err(DIDError::UnsupportedMethod(parsed.method));
        }

        // Validate version matches our resolver
        if parsed.id != self.version {
            return Err(DIDError::NotFound(format!(
                "DID version '{}' not found — current version is '{}'",
                parsed.id, self.version
            )));
        }

        Ok(self.generate_document(&parsed.did))
    }

    /// Verify a HMAC signature over data.
    ///
    /// Uses the signing key to verify that data was signed by this agent.
    pub fn verify_signature(&self, data: &str, signature: &str) -> Result<(), DIDError> {
        let expected_hmac = compute_hmac(&self.signing_key, data);

        if expected_hmac == signature {
            Ok(())
        } else {
            Err(DIDError::VerificationFailed(
                "HMAC signature mismatch".to_string(),
            ))
        }
    }

    /// Sign data with the agent's signing key.
    pub fn sign(&self, data: &str) -> String {
        compute_hmac(&self.signing_key, data)
    }

    /// Parse a DID string into its components.
    ///
    /// Handles both plain DIDs (`did:zeroclaw:0.1.0`) and
    /// DIDs with fragments (`did:zeroclaw:0.1.0#agent`).
    fn parse_did(did: &str) -> Result<ParsedDID, DIDError> {
        // DID format: did:<method>:<id> or did:<method>:<id>#<fragment>
        let Some(without_scheme) = did.strip_prefix("did:") else {
            return Err(DIDError::InvalidDID(format!(
                "DID must start with 'did:', got '{}'",
                did
            )));
        };

        // Strip fragment if present
        let (without_fragment, _fragment) = without_scheme
            .split_once('#')
            .unwrap_or((without_scheme, ""));

        let parts: Vec<&str> = without_fragment.splitn(2, ':').collect();
        if parts.len() < 2 {
            return Err(DIDError::InvalidDID(format!(
                "DID must have at least method and id: '{}'",
                did
            )));
        }

        let method = parts[0];
        let id = parts[1].to_string();

        Ok(ParsedDID {
            did: did.to_string(),
            method: method.to_string(),
            id,
        })
    }

    /// Generate a DID Document for a parsed DID.
    fn generate_document(&self, did: &str) -> DIDDocument {
        let verification_method_id = format!("{}#audit-chain", did);
        let authentication_id = format!("{}#agent", did);

        DIDDocument {
            context: vec![
                "https://www.w3.org/ns/did/v1".to_string(),
                "https://zeroclaw.dev/did/v1".to_string(),
            ],
            id: did.to_string(),
            verification_method: vec![VerificationMethod {
                id: verification_method_id.clone(),
                verification_type: "HmacVerificationKey2024".to_string(),
                controller: did.to_string(),
            }],
            authentication: vec![authentication_id],
            assertion_method: vec![verification_method_id],
        }
    }
}

/// Parsed DID components.
struct ParsedDID {
    did: String,
    method: String,
    id: String,
}

/// Agent identity information.
#[derive(Debug, Clone)]
pub struct AgentIdentity {
    /// DID-style identifier for the agent
    pub did: String,
    /// Human-readable name
    pub name: String,
    /// ZeroClaw version
    pub version: String,
}

impl AgentIdentity {
    /// Create identity from version string.
    pub fn new(version: &str) -> Self {
        Self {
            did: format!("did:zeroclaw:{version}"),
            name: "ZeroClaw".to_string(),
            version: version.to_string(),
        }
    }
}

/// ZeroClaw version constant.
pub const ZEROCLAW_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    fn test_resolver() -> DIDResolver {
        let signing_key = vec![0u8; 32];
        DIDResolver::new("0.1.0".to_string(), signing_key)
    }

    #[test]
    fn resolve_valid_did() {
        let resolver = test_resolver();
        let doc = resolver.resolve("did:zeroclaw:0.1.0").unwrap();

        assert_eq!(doc.id, "did:zeroclaw:0.1.0");
        assert_eq!(doc.context.len(), 2);
        assert_eq!(doc.verification_method.len(), 1);
        assert_eq!(
            doc.verification_method[0].id,
            "did:zeroclaw:0.1.0#audit-chain"
        );
        assert_eq!(
            doc.verification_method[0].verification_type,
            "HmacVerificationKey2024"
        );
        assert_eq!(doc.authentication, vec!["did:zeroclaw:0.1.0#agent"]);
        assert_eq!(
            doc.assertion_method,
            vec!["did:zeroclaw:0.1.0#audit-chain"]
        );
    }

    #[test]
    fn resolve_invalid_did_format() {
        let resolver = test_resolver();
        let result = resolver.resolve("did:invalid");
        assert!(matches!(result, Err(DIDError::InvalidDID(_))));

        let result = resolver.resolve("not-a-did");
        assert!(matches!(result, Err(DIDError::InvalidDID(_))));
    }

    #[test]
    fn resolve_unsupported_method() {
        let resolver = test_resolver();
        let result = resolver.resolve("did:key:abc123");
        assert!(matches!(result, Err(DIDError::UnsupportedMethod(_))));
    }

    #[test]
    fn resolve_wrong_version() {
        let resolver = test_resolver();
        let result = resolver.resolve("did:zeroclaw:99.99.99");
        assert!(matches!(result, Err(DIDError::NotFound(_))));
    }

    #[test]
    fn sign_and_verify() {
        let resolver = test_resolver();
        let data = "test data";
        let signature = resolver.sign(data);

        assert!(resolver.verify_signature(data, &signature).is_ok());
        assert!(resolver.verify_signature(data, "wrong").is_err());
    }

    #[test]
    fn agent_did() {
        let resolver = test_resolver();
        assert_eq!(resolver.agent_did(), "did:zeroclaw:0.1.0");
    }

    #[test]
    fn agent_identity_new() {
        let identity = AgentIdentity::new("1.0.0");
        assert_eq!(identity.did, "did:zeroclaw:1.0.0");
        assert_eq!(identity.name, "ZeroClaw");
        assert_eq!(identity.version, "1.0.0");
    }

    #[test]
    fn compute_hmac_works() {
        let key = vec![0u8; 32];
        let result = compute_hmac(&key, "hello");
        assert_eq!(result.len(), 64); // SHA256 hex is 64 chars
    }

    #[test]
    fn compute_sha256_works() {
        let result = compute_sha256("hello");
        assert_eq!(result.len(), 64); // SHA256 hex is 64 chars
    }
}
