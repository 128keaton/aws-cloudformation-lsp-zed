use serde::{Deserialize, Serialize};
use serde_json::Number;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfnLintInitializationSettings {
    #[serde(rename = "maxRetries")]
    pub max_retries: Option<Number>,

    #[serde(rename = "initialDelayMs")]
    pub initial_delay_ms: Option<Number>,

    #[serde(rename = "maxDelayMs")]
    pub max_delay_ms: Option<Number>,

    #[serde(rename = "backoffMultiplier")]
    pub backoff_multiplier: Option<Number>,

    #[serde(rename = "totalTimeoutMs")]
    pub total_timeout_ms: Option<Number>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfnLintSettings {
    pub enabled: Option<bool>,

    #[serde(rename = "lintOnChange")]
    pub lint_on_change: Option<bool>,

    pub initialization: Option<CfnLintInitializationSettings>,

    #[serde(rename = "ignoreChecks")]
    pub ignore_checks: Option<Vec<String>>,

    #[serde(rename = "includeChecks")]
    pub include_checks: Option<Vec<String>>,

    #[serde(rename = "mandatoryChecks")]
    pub mandatory_checks: Option<Vec<String>>,

    #[serde(rename = "includeExperimental")]
    pub include_experimental: Option<bool>,

    #[serde(rename = "configureRules")]
    pub configure_rules: Option<Vec<String>>,

    pub regions: Option<Vec<String>>,

    #[serde(rename = "customRules")]
    pub custom_rules: Option<Vec<String>>,

    #[serde(rename = "appendRules")]
    pub append_rules: Option<Vec<String>>,

    #[serde(rename = "overrideSpec")]
    pub override_spec: Option<String>,

    #[serde(rename = "registrySchemas")]
    pub registry_schemas: Option<Vec<String>>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardSettings {
    pub enabled: Option<bool>,

    #[serde(rename = "delayMs")]
    pub delay_ms: Option<Number>,
    #[serde(rename = "validateOnChange")]
    pub validate_on_change: Option<bool>,

    #[serde(rename = "enabledRulePacks")]
    pub enabled_rule_packs: Option<Vec<String>>,

    pub timeout: Option<Number>,

    #[serde(
        rename = "maxConcurrentValidations",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrent_validations: Option<Number>,

    #[serde(rename = "maxQueueSize")]
    pub max_queue_size: Option<Number>,

    #[serde(rename = "memoryCleanupInterval")]
    pub memory_cleanup_interval: Option<Number>,

    #[serde(rename = "maxMemoryUsage")]
    pub max_memory_usage: Option<Number>,

    #[serde(rename = "defaultSeverity")]
    pub default_severity: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub region: Option<String>,
    pub profile: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSettings {
    #[serde(rename = "cfnLint")]
    pub cfn_lint: Option<CfnLintSettings>,

    #[serde(rename = "cfnGuard")]
    pub cfn_guard: Option<GuardSettings>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverSettings {
    pub enabled: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionSettings {
    #[serde(rename = "maxCompletions")]
    pub max_completions: Option<Number>,

    pub enabled: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    #[serde(rename = "tabSize")]
    pub tab_size: Option<Number>,

    #[serde(rename = "insertSpaces")]
    pub insert_spaces: Option<bool>,

    #[serde(rename = "detectIndentation")]
    pub detect_indentation: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudformationMeta {
    #[serde(rename = "minDelay")]
    pub min_delay: Option<Number>,

    #[serde(rename = "maxDelay")]
    pub max_delay: Option<Number>,

    #[serde(rename = "maxWaitTime")]
    pub max_wait_time: Option<Number>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cloudformation {
    #[serde(rename = "changeSet")]
    pub change_set: Option<CloudformationMeta>,

    pub stack: Option<CloudformationMeta>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsClientSettings {
    pub cloudformation: Option<Cloudformation>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub profile: Option<ProfileSettings>,
    pub hover: Option<HoverSettings>,
    pub completion: Option<CompletionSettings>,
    pub diagnostics: Option<DiagnosticsSettings>,
    pub editor: Option<EditorSettings>,

    #[serde(rename = "awsClient")]
    pub aws_client: Option<AwsClientSettings>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudformationInitializationOptions {
    pub endpoint: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfoExtension {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub extension: Option<ClientInfoExtension>,

    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOptions {
    #[serde(rename = "staleDaysThreshold")]
    pub stale_days_threshold: Option<Number>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializationFeatureFlags {
    #[serde(rename = "refreshIntervalMs")]
    pub refresh_interval_ms: Option<Number>,

    #[serde(rename = "dynamicRefreshIntervalMs")]
    pub dynamic_refresh_interval_ms: Option<Number>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializationOptions {
    pub settings: Option<Settings>,
    pub schema: Option<SchemaOptions>,
    pub cloudformation: Option<CloudformationInitializationOptions>,

    #[serde(rename = "clientInfo")]
    pub client_info: Option<ClientInfo>,

    #[serde(rename = "telemetryEnabled")]
    pub telemetry_enabled: Option<bool>,

    #[serde(rename = "logLevel")]
    pub log_level: Option<LogLevel>,

    #[serde(rename = "featureFlags")]
    pub feature_flags: Option<InitializationFeatureFlags>,

    #[serde(rename = "storageDir")]
    pub storage_dir: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
