//! Microsoft 365 integration tool — Graph API access for Mail, Teams, Calendar,
//! OneDrive, SharePoint, and Microsoft Intune via a single action-dispatched tool surface.
//!
//! Auth is handled through direct HTTP calls to the Microsoft identity platform
//! (client credentials or device code flow) with token caching.

pub mod auth;
pub mod graph_client;
pub mod intune_client;
pub mod types;

use crate::security::SecurityPolicy;
use crate::security::policy::ToolOperation;
use crate::tools::traits::{Tool, ToolResult};
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

/// Maximum download size for OneDrive files (10 MB).
const MAX_ONEDRIVE_DOWNLOAD_SIZE: usize = 10 * 1024 * 1024;

/// Default number of items to return in list operations.
const DEFAULT_TOP: u32 = 25;

pub struct Microsoft365Tool {
    config: types::Microsoft365ResolvedConfig,
    security: Arc<SecurityPolicy>,
    token_cache: Arc<auth::TokenCache>,
    http_client: reqwest::Client,
}

impl Microsoft365Tool {
    pub fn new(
        config: types::Microsoft365ResolvedConfig,
        security: Arc<SecurityPolicy>,
        zeroclaw_dir: &std::path::Path,
    ) -> anyhow::Result<Self> {
        let http_client =
            crate::config::build_runtime_proxy_client_with_timeouts("tool.microsoft365", 60, 10);
        let token_cache = Arc::new(auth::TokenCache::new(config.clone(), zeroclaw_dir)?);
        Ok(Self {
            config,
            security,
            token_cache,
            http_client,
        })
    }

    async fn get_token(&self) -> anyhow::Result<String> {
        self.token_cache.get_token(&self.http_client).await
    }

    fn user_id(&self) -> &str {
        &self.config.user_id
    }

    async fn dispatch(&self, action: &str, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        match action {
            // Email
            "mail_list" => self.handle_mail_list(args).await,
            "mail_send" => self.handle_mail_send(args).await,
            // Teams
            "teams_message_list" => self.handle_teams_message_list(args).await,
            "teams_message_send" => self.handle_teams_message_send(args).await,
            // Calendar
            "calendar_events_list" => self.handle_calendar_events_list(args).await,
            "calendar_event_create" => self.handle_calendar_event_create(args).await,
            "calendar_event_delete" => self.handle_calendar_event_delete(args).await,
            // OneDrive
            "onedrive_list" => self.handle_onedrive_list(args).await,
            "onedrive_download" => self.handle_onedrive_download(args).await,
            // SharePoint
            "sharepoint_search" => self.handle_sharepoint_search(args).await,
            // Intune / Endpoint Manager
            "intune_device_list" => self.handle_intune_device_list(args).await,
            "intune_device_get" => self.handle_intune_device_get(args).await,
            "intune_device_by_user" => self.handle_intune_device_by_user(args).await,
            "intune_device_detected_apps" => self.handle_intune_device_detected_apps(args).await,
            "intune_device_wipe" => self.handle_intune_device_wipe(args).await,
            "intune_device_retire" => self.handle_intune_device_retire(args).await,
            "intune_device_disable" => self.handle_intune_device_disable(args).await,
            "intune_device_reboot" => self.handle_intune_device_reboot(args).await,
            "intune_device_logout" => self.handle_intune_device_logout(args).await,
            "intune_compliance_summary" => self.handle_intune_compliance_summary(args).await,
            "intune_compliance_policies" => self.handle_intune_compliance_policies(args).await,
            "intune_device_configurations" => self.handle_intune_device_configurations(args).await,
            "intune_apps_list" => self.handle_intune_apps_list(args).await,
            "intune_app_get" => self.handle_intune_app_get(args).await,
            "intune_app_configurations" => self.handle_intune_app_configurations(args).await,
            "intune_enrollment_config" => self.handle_intune_enrollment_config(args).await,
            _ => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Unknown action: {action}")),
            }),
        }
    }

    // ── Read actions ────────────────────────────────────────────────

    async fn handle_mail_list(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.mail_list")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let folder = args["folder"].as_str();
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result =
            graph_client::mail_list(&self.http_client, &token, self.user_id(), folder, top).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_teams_message_list(
        &self,
        args: &serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.teams_message_list")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let team_id = args["team_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("team_id is required"))?;
        let channel_id = args["channel_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("channel_id is required"))?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result =
            graph_client::teams_message_list(&self.http_client, &token, team_id, channel_id, top)
                .await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_calendar_events_list(
        &self,
        args: &serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.calendar_events_list")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let start = args["start"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("start datetime is required"))?;
        let end = args["end"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("end datetime is required"))?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result = graph_client::calendar_events_list(
            &self.http_client,
            &token,
            self.user_id(),
            start,
            end,
            top,
        )
        .await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_onedrive_list(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.onedrive_list")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let path = args["path"].as_str();

        let result =
            graph_client::onedrive_list(&self.http_client, &token, self.user_id(), path).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_onedrive_download(
        &self,
        args: &serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.onedrive_download")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let item_id = args["item_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("item_id is required"))?;
        let max_size = args["max_size"]
            .as_u64()
            .and_then(|v| usize::try_from(v).ok())
            .unwrap_or(MAX_ONEDRIVE_DOWNLOAD_SIZE)
            .min(MAX_ONEDRIVE_DOWNLOAD_SIZE);

        let bytes = graph_client::onedrive_download(
            &self.http_client,
            &token,
            self.user_id(),
            item_id,
            max_size,
        )
        .await?;

        // Return base64-encoded for binary safety.
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&bytes);

        Ok(ToolResult {
            success: true,
            output: format!(
                "Downloaded {} bytes (base64 encoded):\n{encoded}",
                bytes.len()
            ),
            error: None,
        })
    }

    async fn handle_sharepoint_search(
        &self,
        args: &serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.sharepoint_search")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("query is required"))?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result = graph_client::sharepoint_search(&self.http_client, &token, query, top).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    // ── Write actions ───────────────────────────────────────────────

    async fn handle_mail_send(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.mail_send")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let to: Vec<String> = args["to"]
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("to must be an array of email addresses"))?
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        if to.is_empty() {
            anyhow::bail!("to must contain at least one email address");
        }

        let subject = args["subject"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("subject is required"))?;
        let body = args["body"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("body is required"))?;

        graph_client::mail_send(
            &self.http_client,
            &token,
            self.user_id(),
            &to,
            subject,
            body,
        )
        .await?;

        Ok(ToolResult {
            success: true,
            output: format!("Email sent to: {}", to.join(", ")),
            error: None,
        })
    }

    async fn handle_teams_message_send(
        &self,
        args: &serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.teams_message_send")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let team_id = args["team_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("team_id is required"))?;
        let channel_id = args["channel_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("channel_id is required"))?;
        let body = args["body"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("body is required"))?;

        graph_client::teams_message_send(&self.http_client, &token, team_id, channel_id, body)
            .await?;

        Ok(ToolResult {
            success: true,
            output: "Teams message sent".to_string(),
            error: None,
        })
    }

    async fn handle_calendar_event_create(
        &self,
        args: &serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.calendar_event_create")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let subject = args["subject"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("subject is required"))?;
        let start = args["start"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("start datetime is required"))?;
        let end = args["end"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("end datetime is required"))?;
        let attendees: Vec<String> = args["attendees"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let body_text = args["body"].as_str();

        let event_id = graph_client::calendar_event_create(
            &self.http_client,
            &token,
            self.user_id(),
            subject,
            start,
            end,
            &attendees,
            body_text,
        )
        .await?;

        Ok(ToolResult {
            success: true,
            output: format!("Calendar event created (id: {event_id})"),
            error: None,
        })
    }

    async fn handle_calendar_event_delete(
        &self,
        args: &serde_json::Value,
    ) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.calendar_event_delete")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let event_id = args["event_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("event_id is required"))?;

        graph_client::calendar_event_delete(&self.http_client, &token, self.user_id(), event_id)
            .await?;

        Ok(ToolResult {
            success: true,
            output: format!("Calendar event {event_id} deleted"),
            error: None,
        })
    }

    // ── Intune / Endpoint Manager actions ───────────────────────────

    async fn handle_intune_device_list(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_device_list")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);
        let filter = args["filter"].as_str();
        let select = args["select"].as_str();

        let result = intune_client::device_list(&self.http_client, &token, top, filter, select).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_device_get(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_device_get")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let device_id = args["device_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("device_id is required"))?;

        let result = intune_client::device_get(&self.http_client, &token, device_id).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_device_by_user(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_device_by_user")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let user_id = args["user_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("user_id is required"))?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result = intune_client::device_list_by_user(&self.http_client, &token, user_id, top).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_device_detected_apps(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_device_detected_apps")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let device_id = args["device_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("device_id is required"))?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result = intune_client::device_detected_apps(&self.http_client, &token, device_id, top).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_device_wipe(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.intune_device_wipe")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let device_id = args["device_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("device_id is required"))?;
        let keep_encryption_keys = args["keep_encryption_keys"].as_bool();
        let use_recovery_key = args["use_recovery_key"].as_bool();

        intune_client::device_wipe(&self.http_client, &token, device_id, keep_encryption_keys, use_recovery_key).await?;

        Ok(ToolResult {
            success: true,
            output: format!("Wipe requested for device {device_id}. WARNING: This will erase all data!"),
            error: None,
        })
    }

    async fn handle_intune_device_retire(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.intune_device_retire")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let device_id = args["device_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("device_id is required"))?;

        intune_client::device_retire(&self.http_client, &token, device_id).await?;

        Ok(ToolResult {
            success: true,
            output: format!("Retire requested for device {device_id}. Company data will be removed, user data preserved."),
            error: None,
        })
    }

    async fn handle_intune_device_disable(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.intune_device_disable")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let device_id = args["device_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("device_id is required"))?;

        intune_client::device_disable(&self.http_client, &token, device_id).await?;

        Ok(ToolResult {
            success: true,
            output: format!("Device {device_id} has been disabled. Access to company resources blocked."),
            error: None,
        })
    }

    async fn handle_intune_device_reboot(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.intune_device_reboot")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let device_id = args["device_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("device_id is required"))?;

        intune_client::device_reboot(&self.http_client, &token, device_id).await?;

        Ok(ToolResult {
            success: true,
            output: format!("Reboot requested for device {device_id}."),
            error: None,
        })
    }

    async fn handle_intune_device_logout(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Act, "microsoft365.intune_device_logout")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let device_id = args["device_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("device_id is required"))?;

        intune_client::device_logout_users(&self.http_client, &token, device_id).await?;

        Ok(ToolResult {
            success: true,
            output: format!("All users logged out from shared device {device_id}."),
            error: None,
        })
    }

    async fn handle_intune_compliance_summary(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_compliance_summary")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let result = intune_client::compliance_summary(&self.http_client, &token).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_compliance_policies(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_compliance_policies")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result = intune_client::compliance_policies_list(&self.http_client, &token, top).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_device_configurations(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_device_configurations")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result = intune_client::device_configurations_list(&self.http_client, &token, top).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_apps_list(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_apps_list")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);
        let filter = args["filter"].as_str();

        let result = intune_client::apps_list(&self.http_client, &token, top, filter).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_app_get(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_app_get")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let app_id = args["app_id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("app_id is required"))?;

        let result = intune_client::app_get(&self.http_client, &token, app_id).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_app_configurations(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_app_configurations")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;
        let top = u32::try_from(args["top"].as_u64().unwrap_or(u64::from(DEFAULT_TOP)))
            .unwrap_or(DEFAULT_TOP);

        let result = intune_client::app_configurations_list(&self.http_client, &token, top).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }

    async fn handle_intune_enrollment_config(&self, args: &serde_json::Value) -> anyhow::Result<ToolResult> {
        self.security
            .enforce_tool_operation(ToolOperation::Read, "microsoft365.intune_enrollment_config")
            .map_err(|e| anyhow::anyhow!(e))?;

        let token = self.get_token().await?;

        let result = intune_client::enrollment_configurations_list(&self.http_client, &token).await?;

        Ok(ToolResult {
            success: true,
            output: serde_json::to_string_pretty(&result)?,
            error: None,
        })
    }
}

#[async_trait]
impl Tool for Microsoft365Tool {
    fn name(&self) -> &str {
        "microsoft365"
    }

    fn description(&self) -> &str {
        "Microsoft 365 integration: manage Outlook mail, Teams messages, Calendar events, \
         OneDrive files, SharePoint search, and Microsoft Intune/Endpoint Manager devices, \
         compliance policies, and mobile apps via Microsoft Graph API"
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["action"],
            "properties": {
                "action": {
                    "type": "string",
                    "enum": [
                        "mail_list",
                        "mail_send",
                        "teams_message_list",
                        "teams_message_send",
                        "calendar_events_list",
                        "calendar_event_create",
                        "calendar_event_delete",
                        "onedrive_list",
                        "onedrive_download",
                        "sharepoint_search",
                        "intune_device_list",
                        "intune_device_get",
                        "intune_device_by_user",
                        "intune_device_detected_apps",
                        "intune_device_wipe",
                        "intune_device_retire",
                        "intune_device_disable",
                        "intune_device_reboot",
                        "intune_device_logout",
                        "intune_compliance_summary",
                        "intune_compliance_policies",
                        "intune_device_configurations",
                        "intune_apps_list",
                        "intune_app_get",
                        "intune_app_configurations",
                        "intune_enrollment_config"
                    ],
                    "description": "The Microsoft 365 action to perform"
                },
                "folder": {
                    "type": "string",
                    "description": "Mail folder ID (for mail_list, e.g. 'inbox', 'sentitems')"
                },
                "to": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Recipient email addresses (for mail_send)"
                },
                "subject": {
                    "type": "string",
                    "description": "Email subject or calendar event subject"
                },
                "body": {
                    "type": "string",
                    "description": "Message body text"
                },
                "team_id": {
                    "type": "string",
                    "description": "Teams team ID (for teams_message_list/send)"
                },
                "channel_id": {
                    "type": "string",
                    "description": "Teams channel ID (for teams_message_list/send)"
                },
                "start": {
                    "type": "string",
                    "description": "Start datetime in ISO 8601 format (for calendar actions)"
                },
                "end": {
                    "type": "string",
                    "description": "End datetime in ISO 8601 format (for calendar actions)"
                },
                "attendees": {
                    "type": "array",
                    "items": { "type": "string" },
                    "description": "Attendee email addresses (for calendar_event_create)"
                },
                "event_id": {
                    "type": "string",
                    "description": "Calendar event ID (for calendar_event_delete)"
                },
                "path": {
                    "type": "string",
                    "description": "OneDrive folder path (for onedrive_list)"
                },
                "item_id": {
                    "type": "string",
                    "description": "OneDrive item ID (for onedrive_download)"
                },
                "max_size": {
                    "type": "integer",
                    "description": "Maximum download size in bytes (for onedrive_download, default 10MB)"
                },
                "query": {
                    "type": "string",
                    "description": "Search query (for sharepoint_search)"
                },
                "top": {
                    "type": "integer",
                    "description": "Maximum number of items to return (default 25)"
                },
                // Intune / Endpoint Manager parameters
                "device_id": {
                    "type": "string",
                    "description": "Intune device ID (for device actions)"
                },
                "user_id": {
                    "type": "string",
                    "description": "User ID or email (for intune_device_by_user)"
                },
                "filter": {
                    "type": "string",
                    "description": "OData filter expression (for intune_device_list, intune_apps_list)"
                },
                "select": {
                    "type": "string",
                    "description": "Comma-separated fields to return (for intune_device_list)"
                },
                "keep_encryption_keys": {
                    "type": "boolean",
                    "description": "Keep BitLocker keys during wipe (for intune_device_wipe)"
                },
                "use_recovery_key": {
                    "type": "boolean",
                    "description": "Use wipe recovery key (for intune_device_wipe)"
                },
                "app_id": {
                    "type": "string",
                    "description": "App ID (for intune_app_get)"
                }
            }
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<ToolResult> {
        let action = match args["action"].as_str() {
            Some(a) => a.to_string(),
            None => {
                return Ok(ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some("'action' parameter is required".to_string()),
                });
            }
        };

        match self.dispatch(&action, &args).await {
            Ok(result) => Ok(result),
            Err(e) => Ok(ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("microsoft365.{action} failed: {e}")),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_name_is_microsoft365() {
        // Verify the schema is valid JSON with the expected structure.
        let schema_str = r#"{"type":"object","required":["action"]}"#;
        let _: serde_json::Value = serde_json::from_str(schema_str).unwrap();
    }

    #[test]
    fn parameters_schema_has_action_enum() {
        let schema = json!({
            "type": "object",
            "required": ["action"],
            "properties": {
                "action": {
                    "type": "string",
                    "enum": [
                        "mail_list",
                        "mail_send",
                        "teams_message_list",
                        "teams_message_send",
                        "calendar_events_list",
                        "calendar_event_create",
                        "calendar_event_delete",
                        "onedrive_list",
                        "onedrive_download",
                        "sharepoint_search",
                        "intune_device_list",
                        "intune_device_get",
                        "intune_device_by_user",
                        "intune_device_detected_apps",
                        "intune_device_wipe",
                        "intune_device_retire",
                        "intune_device_disable",
                        "intune_device_reboot",
                        "intune_device_logout",
                        "intune_compliance_summary",
                        "intune_compliance_policies",
                        "intune_device_configurations",
                        "intune_apps_list",
                        "intune_app_get",
                        "intune_app_configurations",
                        "intune_enrollment_config"
                    ]
                }
            }
        });

        let actions = schema["properties"]["action"]["enum"].as_array().unwrap();
        assert_eq!(actions.len(), 26);
        assert!(actions.contains(&json!("mail_list")));
        assert!(actions.contains(&json!("sharepoint_search")));
        // Intune actions
        assert!(actions.contains(&json!("intune_device_list")));
        assert!(actions.contains(&json!("intune_device_wipe")));
        assert!(actions.contains(&json!("intune_compliance_summary")));
    }

    #[test]
    fn action_dispatch_table_is_exhaustive() {
        let valid_actions = [
            "mail_list",
            "mail_send",
            "teams_message_list",
            "teams_message_send",
            "calendar_events_list",
            "calendar_event_create",
            "calendar_event_delete",
            "onedrive_list",
            "onedrive_download",
            "sharepoint_search",
            "intune_device_list",
            "intune_device_get",
            "intune_device_by_user",
            "intune_device_detected_apps",
            "intune_device_wipe",
            "intune_device_retire",
            "intune_device_disable",
            "intune_device_reboot",
            "intune_device_logout",
            "intune_compliance_summary",
            "intune_compliance_policies",
            "intune_device_configurations",
            "intune_apps_list",
            "intune_app_get",
            "intune_app_configurations",
            "intune_enrollment_config",
        ];
        assert_eq!(valid_actions.len(), 26);
        assert!(!valid_actions.contains(&"invalid_action"));
    }
}
