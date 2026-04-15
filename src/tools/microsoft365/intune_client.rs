//! Intune/Microsoft Endpoint Manager Graph API client functions.
//!
//! Provides CRUD operations for managed devices, compliance policies,
//! applications, and remote actions (wipe, retire, disable, reboot).
//!
//! All endpoints use the beta API version as Intune management APIs
//! are primarily available there.
//!
//! API Reference: <https://learn.microsoft.com/en-us/graph/api/resources/intune-graph-overview>

use anyhow::Context;

const GRAPH_BETA: &str = "https://graph.microsoft.com/beta";

/// Percent-encode a path segment to prevent path-traversal attacks.
fn encode_path_segment(segment: &str) -> String {
    urlencoding::encode(segment).into_owned()
}

// ── Managed Devices ─────────────────────────────────────────────────────────────

/// List all managed devices in the organization.
pub async fn device_list(
    client: &reqwest::Client,
    token: &str,
    top: u32,
    filter: Option<&str>,
    select: Option<&str>,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/managedDevices");
    let mut params = vec![("$top".to_string(), top.to_string())];

    if let Some(f) = filter {
        params.push(("$filter".to_string(), f.to_string()));
    }
    if let Some(s) = select {
        params.push(("$select".to_string(), s.to_string()));
    }

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&params.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<_>>())
        .send()
        .await
        .context("intune: device_list request failed")?;

    handle_json_response(resp, "device_list").await
}

/// Get a specific managed device by its ID.
pub async fn device_get(
    client: &reqwest::Client,
    token: &str,
    device_id: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/managedDevices/{}",
        encode_path_segment(device_id)
    );

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: device_get request failed")?;

    handle_json_response(resp, "device_get").await
}

/// Get managed devices assigned to a specific user.
pub async fn device_list_by_user(
    client: &reqwest::Client,
    token: &str,
    user_id: &str,
    top: u32,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{GRAPH_BETA}/users/{}/managedDevices",
        encode_path_segment(user_id)
    );

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("$top", top.to_string())])
        .send()
        .await
        .context("intune: device_list_by_user request failed")?;

    handle_json_response(resp, "device_list_by_user").await
}

/// Get applications detected on a managed device.
pub async fn device_detected_apps(
    client: &reqwest::Client,
    token: &str,
    device_id: &str,
    top: u32,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/managedDevices/{}/detectedApps",
        encode_path_segment(device_id)
    );

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("$top", top.to_string())])
        .send()
        .await
        .context("intune: device_detected_apps request failed")?;

    handle_json_response(resp, "device_detected_apps").await
}

/// Request a remote wipe of a managed device.
/// WARNING: This will factory-reset the device and erase all data.
pub async fn device_wipe(
    client: &reqwest::Client,
    token: &str,
    device_id: &str,
    keep_encryption_keys: Option<bool>,
    use_recovery_key: Option<bool>,
) -> anyhow::Result<()> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/managedDevices/{}/wipe",
        encode_path_segment(device_id)
    );

    let mut payload = serde_json::json!({});
    if let Some(keep) = keep_encryption_keys {
        payload["keepEncryptionKeys"] = serde_json::json!(keep);
    }
    if let Some(recovery) = use_recovery_key {
        payload["useWipeRecoveryKey"] = serde_json::json!(recovery);
    }

    let resp = client
        .post(&url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .context("intune: device_wipe request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let code = extract_graph_error_code(&body).unwrap_or_else(|| "unknown".to_string());
        tracing::debug!("intune: device_wipe raw error body: {body}");
        anyhow::bail!("intune: device_wipe failed ({status}, code={code})");
    }

    Ok(())
}

/// Request a remote retire of a managed device.
/// This removes company data while preserving user data.
pub async fn device_retire(
    client: &reqwest::Client,
    token: &str,
    device_id: &str,
) -> anyhow::Result<()> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/managedDevices/{}/retire",
        encode_path_segment(device_id)
    );

    let resp = client
        .post(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: device_retire request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let code = extract_graph_error_code(&body).unwrap_or_else(|| "unknown".to_string());
        tracing::debug!("intune: device_retire raw error body: {body}");
        anyhow::bail!("intune: device_retire failed ({status}, code={code})");
    }

    Ok(())
}

/// Disable a managed device (blocks access to company resources).
pub async fn device_disable(
    client: &reqwest::Client,
    token: &str,
    device_id: &str,
) -> anyhow::Result<()> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/managedDevices/{}/disable",
        encode_path_segment(device_id)
    );

    let resp = client
        .post(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: device_disable request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let code = extract_graph_error_code(&body).unwrap_or_else(|| "unknown".to_string());
        tracing::debug!("intune: device_disable raw error body: {body}");
        anyhow::bail!("intune: device_disable failed ({status}, code={code})");
    }

    Ok(())
}

/// Request a remote reboot of a managed device.
pub async fn device_reboot(
    client: &reqwest::Client,
    token: &str,
    device_id: &str,
) -> anyhow::Result<()> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/managedDevices/{}/reboot",
        encode_path_segment(device_id)
    );

    let resp = client
        .post(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: device_reboot request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let code = extract_graph_error_code(&body).unwrap_or_else(|| "unknown".to_string());
        tracing::debug!("intune: device_reboot raw error body: {body}");
        anyhow::bail!("intune: device_reboot failed ({status}, code={code})");
    }

    Ok(())
}

/// Request a remote logout of all users from a shared device.
pub async fn device_logout_users(
    client: &reqwest::Client,
    token: &str,
    device_id: &str,
) -> anyhow::Result<()> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/managedDevices/{}/logoutSharedAppleDevicePrimaryUser",
        encode_path_segment(device_id)
    );

    let resp = client
        .post(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: device_logout_users request failed")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let code = extract_graph_error_code(&body).unwrap_or_else(|| "unknown".to_string());
        tracing::debug!("intune: device_logout_users raw error body: {body}");
        anyhow::bail!("intune: device_logout_users failed ({status}, code={code})");
    }

    Ok(())
}

// ── Compliance Policies ────────────────────────────────────────────────────────

/// Get device compliance policy summary for the organization.
pub async fn compliance_summary(
    client: &reqwest::Client,
    token: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/deviceCompliancePolicySummarization");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: compliance_summary request failed")?;

    handle_json_response(resp, "compliance_summary").await
}

/// List all device compliance policies.
pub async fn compliance_policies_list(
    client: &reqwest::Client,
    token: &str,
    top: u32,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/deviceCompliancePolicies");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("$top", top.to_string())])
        .send()
        .await
        .context("intune: compliance_policies_list request failed")?;

    handle_json_response(resp, "compliance_policies_list").await
}

/// Get setting summary for device compliance.
pub async fn compliance_setting_summary(
    client: &reqwest::Client,
    token: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/deviceComplianceSettingSummaries");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: compliance_setting_summary request failed")?;

    handle_json_response(resp, "compliance_setting_summary").await
}

// ── Device Configurations ──────────────────────────────────────────────────────

/// List all device configurations.
pub async fn device_configurations_list(
    client: &reqwest::Client,
    token: &str,
    top: u32,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/deviceConfigurations");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("$top", top.to_string())])
        .send()
        .await
        .context("intune: device_configurations_list request failed")?;

    handle_json_response(resp, "device_configurations_list").await
}

/// Get a specific device configuration by ID.
pub async fn device_configuration_get(
    client: &reqwest::Client,
    token: &str,
    config_id: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{GRAPH_BETA}/deviceManagement/deviceConfigurations/{}",
        encode_path_segment(config_id)
    );

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: device_configuration_get request failed")?;

    handle_json_response(resp, "device_configuration_get").await
}

// ── Mobile Apps ────────────────────────────────────────────────────────────────

/// List all mobile applications.
pub async fn apps_list(
    client: &reqwest::Client,
    token: &str,
    top: u32,
    filter: Option<&str>,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceAppManagement/mobileApps");
    let mut params = vec![("$top".to_string(), top.to_string())];

    if let Some(f) = filter {
        params.push(("$filter".to_string(), f.to_string()));
    }

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&params.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<_>>())
        .send()
        .await
        .context("intune: apps_list request failed")?;

    handle_json_response(resp, "apps_list").await
}

/// Get a specific mobile app by ID.
pub async fn app_get(
    client: &reqwest::Client,
    token: &str,
    app_id: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{GRAPH_BETA}/deviceAppManagement/mobileApps/{}",
        encode_path_segment(app_id)
    );

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: app_get request failed")?;

    handle_json_response(resp, "app_get").await
}

/// List app assignments (which groups/devices have which apps assigned).
pub async fn app_assignments_list(
    client: &reqwest::Client,
    token: &str,
    app_id: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!(
        "{GRAPH_BETA}/deviceAppManagement/mobileApps/{}/assignments",
        encode_path_segment(app_id)
    );

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: app_assignments_list request failed")?;

    handle_json_response(resp, "app_assignments_list").await
}

/// Get managed app registrations (information about app usage).
pub async fn app_registrations_list(
    client: &reqwest::Client,
    token: &str,
    top: u32,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceAppManagement/managedAppRegistrations");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("$top", top.to_string())])
        .send()
        .await
        .context("intune: app_registrations_list request failed")?;

    handle_json_response(resp, "app_registrations_list").await
}

// ── App Configurations ─────────────────────────────────────────────────────────

/// List mobile app configurations (app protection/configuration policies).
pub async fn app_configurations_list(
    client: &reqwest::Client,
    token: &str,
    top: u32,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceAppManagement/mobileAppConfigurations");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("$top", top.to_string())])
        .send()
        .await
        .context("intune: app_configurations_list request failed")?;

    handle_json_response(resp, "app_configurations_list").await
}

// ── Management Intents ─────────────────────────────────────────────────────────

/// List management intents (proactive remediations, device configuration).
pub async fn management_intents_list(
    client: &reqwest::Client,
    token: &str,
    top: u32,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/intents");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .query(&[("$top", top.to_string())])
        .send()
        .await
        .context("intune: management_intents_list request failed")?;

    handle_json_response(resp, "management_intents_list").await
}

// ── Enrollment ────────────────────────────────────────────────────────────────

/// List device enrollment configurations.
pub async fn enrollment_configurations_list(
    client: &reqwest::Client,
    token: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/deviceEnrollmentConfigurations");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: enrollment_configurations_list request failed")?;

    handle_json_response(resp, "enrollment_configurations_list").await
}

/// Get enrollment status page information.
pub async fn enrollment_status(
    client: &reqwest::Client,
    token: &str,
) -> anyhow::Result<serde_json::Value> {
    let url = format!("{GRAPH_BETA}/deviceManagement/windowsEnrollmentStatusWindows");

    let resp = client
        .get(&url)
        .bearer_auth(token)
        .send()
        .await
        .context("intune: enrollment_status request failed")?;

    handle_json_response(resp, "enrollment_status").await
}

// ── Utility ───────────────────────────────────────────────────────────────────

/// Extract a short, safe error code from a Graph API JSON error body.
fn extract_graph_error_code(body: &str) -> Option<String> {
    let parsed: serde_json::Value = serde_json::from_str(body).ok()?;
    parsed
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_str())
        .map(|s| s.to_string())
}

/// Parse a JSON response body, returning an error on non-success status.
async fn handle_json_response(
    resp: reqwest::Response,
    operation: &str,
) -> anyhow::Result<serde_json::Value> {
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        let code = extract_graph_error_code(&body).unwrap_or_else(|| "unknown".to_string());
        tracing::debug!("intune: {operation} raw error body: {body}");
        anyhow::bail!("intune: {operation} failed ({status}, code={code})");
    }

    resp.json()
        .await
        .with_context(|| format!("intune: failed to parse {operation} response"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_path_segment_escapes_special_chars() {
        assert_eq!(encode_path_segment("abc-123"), "abc-123");
        assert_eq!(encode_path_segment("user@contoso.com"), "user%40contoso.com");
        assert_eq!(encode_path_segment("path/with/slashes"), "path%2Fwith%2Fslashes");
    }

    #[tokio::test]
    async fn device_list_url_uses_beta() {
        let url = format!("{GRAPH_BETA}/deviceManagement/managedDevices");
        assert!(url.contains("graph.microsoft.com/beta"));
    }

    #[tokio::test]
    async fn wipe_url_format() {
        let device_id = "abc123";
        let url = format!(
            "{GRAPH_BETA}/deviceManagement/managedDevices/{}/wipe",
            encode_path_segment(device_id)
        );
        assert!(url.contains("/wipe"));
    }

    #[tokio::test]
    async fn detected_apps_url_format() {
        let device_id = "device-xyz";
        let url = format!(
            "{GRAPH_BETA}/deviceManagement/managedDevices/{}/detectedApps",
            encode_path_segment(device_id)
        );
        assert!(url.contains("/detectedApps"));
    }
}
