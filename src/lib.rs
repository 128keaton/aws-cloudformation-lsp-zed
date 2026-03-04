use serde::{Deserialize, Serialize};
use serde_json::Number;

use std::path::PathBuf;
use std::{env, fs};
use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfnLintInitializationSettings {
    #[serde(rename = "maxRetries", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<Number>,

    #[serde(rename = "initialDelayMs", skip_serializing_if = "Option::is_none")]
    pub initial_delay_ms: Option<Number>,

    #[serde(rename = "maxDelayMs", skip_serializing_if = "Option::is_none")]
    pub max_delay_ms: Option<Number>,

    #[serde(rename = "backoffMultiplier", skip_serializing_if = "Option::is_none")]
    pub backoff_multiplier: Option<Number>,

    #[serde(rename = "totalTimeoutMs", skip_serializing_if = "Option::is_none")]
    pub total_timeout_ms: Option<Number>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfnLintSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(rename = "lintOnChange", skip_serializing_if = "Option::is_none")]
    pub lint_on_change: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization: Option<CfnLintInitializationSettings>,

    #[serde(rename = "ignoreChecks", skip_serializing_if = "Option::is_none")]
    pub ignore_checks: Option<Vec<String>>,

    #[serde(rename = "includeChecks", skip_serializing_if = "Option::is_none")]
    pub include_checks: Option<Vec<String>>,

    #[serde(rename = "mandatoryChecks", skip_serializing_if = "Option::is_none")]
    pub mandatory_checks: Option<Vec<String>>,

    #[serde(
        rename = "includeExperimental",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_experimental: Option<bool>,

    #[serde(rename = "configureRules", skip_serializing_if = "Option::is_none")]
    pub configure_rules: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,

    #[serde(rename = "customRules", skip_serializing_if = "Option::is_none")]
    pub custom_rules: Option<Vec<String>>,

    #[serde(rename = "appendRules", skip_serializing_if = "Option::is_none")]
    pub append_rules: Option<Vec<String>>,

    #[serde(rename = "overrideSpec", skip_serializing_if = "Option::is_none")]
    pub override_spec: Option<String>,

    #[serde(rename = "registrySchemas", skip_serializing_if = "Option::is_none")]
    pub registry_schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(rename = "delayMs", skip_serializing_if = "Option::is_none")]
    pub delay_ms: Option<Number>,
    #[serde(rename = "validateOnChange", skip_serializing_if = "Option::is_none")]
    pub validate_on_change: Option<bool>,

    #[serde(rename = "enabledRulePacks", skip_serializing_if = "Option::is_none")]
    pub enabled_rule_packs: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Number>,

    #[serde(
        rename = "maxConcurrentValidations",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrent_validations: Option<Number>,

    #[serde(rename = "maxQueueSize", skip_serializing_if = "Option::is_none")]
    pub max_queue_size: Option<Number>,

    #[serde(
        rename = "memoryCleanupInterval",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_cleanup_interval: Option<Number>,

    #[serde(rename = "maxMemoryUsage", skip_serializing_if = "Option::is_none")]
    pub max_memory_usage: Option<Number>,

    #[serde(rename = "defaultSeverity", skip_serializing_if = "Option::is_none")]
    pub default_severity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub region: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsSettings {
    #[serde(rename = "cfnLint")]
    pub cfn_lint: Option<CfnLintSettings>,

    #[serde(rename = "cfnGuard")]
    pub cfn_guard: Option<GuardSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionSettings {
    #[serde(rename = "maxCompletions")]
    pub max_completions: Option<Number>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSettings {
    #[serde(rename = "tabSize")]
    pub tab_size: Option<Number>,

    #[serde(rename = "insertSpaces")]
    pub insert_spaces: Option<bool>,

    #[serde(rename = "detectIndentation")]
    pub detect_indentation: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudformationMeta {
    #[serde(rename = "minDelay")]
    pub min_delay: Option<Number>,

    #[serde(rename = "maxDelay")]
    pub max_delay: Option<Number>,

    #[serde(rename = "maxWaitTime")]
    pub max_wait_time: Option<Number>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cloudformation {
    #[serde(rename = "changeSet", skip_serializing_if = "Option::is_none")]
    pub change_set: Option<CloudformationMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<CloudformationMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsClientSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudformation: Option<Cloudformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ProfileSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<HoverSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<CompletionSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorSettings>,

    #[serde(rename = "awsClient", skip_serializing_if = "Option::is_none")]
    pub aws_client: Option<AwsClientSettings>,
}

pub struct AwsCloudformationExtension {
    cached_binary_path: Option<String>,
}

impl AwsCloudformationExtension {
    fn load_aws_cloudformation_lsp_settings(&self, worktree: &zed::Worktree) -> Settings {
        // Try to load settings from various configuration sources

        // 1. Check for .sonarlint/settings.json in project root
        let worktree_path = PathBuf::from(worktree.root_path());
        let aws_settings_path = worktree_path.join(".aws").join("settings.json");

        if let Ok(settings_content) = fs::read_to_string(&aws_settings_path) {
            if let Ok(settings) = serde_json::from_str::<Settings>(&settings_content) {
                return settings;
            }
        }

        // 2. Check for AWS configuration in .zed/aws_settings.json
        let zed_settings_path = worktree_path.join(".zed").join("aws_settings.json");
        if let Ok(settings_content) = fs::read_to_string(&zed_settings_path) {
            if let Ok(zed_settings) = serde_json::from_str::<serde_json::Value>(&settings_content) {
                if let Some(config) = zed_settings.get("aws") {
                    if let Ok(settings) = serde_json::from_value::<Settings>(config.clone()) {
                        return settings;
                    }
                }
            }
        }

        // 3. Default settings (standalone mode)
        Settings {
            profile: Some(ProfileSettings {
                region: Some("us-east-1".to_string()),
                profile: Some("default".to_string()),
            }),
            hover: Some(HoverSettings {
                enabled: Some(true),
            }),
            completion: Some(CompletionSettings {
                enabled: Some(true),
                max_completions: Some(Number::from(100)),
            }),
            diagnostics: Some(DiagnosticsSettings {
                cfn_lint: Some(CfnLintSettings {
                    enabled: Some(true),
                    lint_on_change: Some(true),
                    initialization: Some(CfnLintInitializationSettings {
                        max_retries: Some(Number::from(3)),
                        initial_delay_ms: Some(Number::from(1000)),
                        max_delay_ms: Some(Number::from(30000)),
                        backoff_multiplier: Some(Number::from(2)),
                        total_timeout_ms: Some(Number::from(120000)),
                    }),
                    ignore_checks: Some(vec![]),
                    include_checks: Some(vec![]),
                    mandatory_checks: Some(vec![]),
                    include_experimental: Some(false),
                    configure_rules: Some(vec![]),
                    regions: Some(vec![]),
                    custom_rules: Some(vec![]),
                    append_rules: Some(vec![]),
                    override_spec: Some("".to_string()),
                    registry_schemas: Some(vec![]),
                }),
                cfn_guard: Some(GuardSettings {
                    enabled: Some(true),
                    delay_ms: Some(Number::from(1000)),
                    validate_on_change: Some(true),
                    enabled_rule_packs: Some(vec!["cis-aws-benchmark-level-1".to_string()]),
                    timeout: Some(Number::from(30000)),
                    max_concurrent_validations: Some(Number::from(3)),
                    max_queue_size: Some(Number::from(10)),
                    memory_cleanup_interval: Some(Number::from(60000)),
                    max_memory_usage: Some(Number::from(100 * 1024 * 1024)),
                    default_severity: Some("information".to_string()),
                }),
            }),
            editor: Some(EditorSettings {
                tab_size: Some(Number::from(2)),
                insert_spaces: Some(true),
                detect_indentation: Some(true),
            }),
            aws_client: Some(AwsClientSettings {
                cloudformation: Some(Cloudformation {
                    change_set: Some(CloudformationMeta {
                        min_delay: Some(Number::from(1)),
                        max_delay: Some(Number::from(8)),
                        max_wait_time: Some(Number::from(600)),
                    }),
                    stack: Some(CloudformationMeta {
                        min_delay: Some(Number::from(3)),
                        max_delay: Some(Number::from(10)),
                        max_wait_time: Some(Number::from(1000)),
                    }),
                }),
            }),
        }
    }

    fn create_config(&self, settings: &Settings) -> serde_json::Value {
        let config = serde_json::json!({
            "profile": {
                "region": settings.profile.as_ref().and_then(|f| f.region.as_ref()).cloned().unwrap_or("us-east-1".to_string()),
                "profile": settings.profile.as_ref().and_then(|f| f.profile.as_ref()).cloned().unwrap_or("default".to_string())
            },
            "hover": {
                "enabled": settings.hover.as_ref().and_then(|f| f.enabled).unwrap_or(true)
            },
            "completion": {
                "enabled": settings.completion.as_ref().and_then(|f| f.enabled).unwrap_or(true),
                "maxCompletions": settings.completion.as_ref().and_then(|f| f.max_completions.as_ref()).cloned().unwrap_or(Number::from(100)),
            },
            "diagnostics": {
                "cfnLint": match settings.diagnostics.as_ref().and_then(|f| f.cfn_lint.as_ref()) {
                    Some(cfn_lint) => serde_json::json!({
                        "enabled": cfn_lint.enabled,
                        "lintOnChange": cfn_lint.lint_on_change,
                        "initialization": cfn_lint.initialization,
                        "ignoreChecks": cfn_lint.ignore_checks,
                        "includeChecks": cfn_lint.include_checks,
                        "mandatoryChecks": cfn_lint.mandatory_checks,
                        "includeExperimental": cfn_lint.include_experimental,
                        "configureRules": cfn_lint.configure_rules,
                        "regions": cfn_lint.regions,
                        "customRules": cfn_lint.custom_rules,
                        "appendRules": cfn_lint.append_rules,
                        "overrideSpec": cfn_lint.override_spec,
                        "registrySchemas": cfn_lint.registry_schemas,
                    }),
                    None => serde_json::Value::Null,
                },
                "cfnGuard": match settings.diagnostics.as_ref().and_then(|f| f.cfn_guard.as_ref()) {
                    Some(cfn_guard) => serde_json::json!({
                        "enabled": cfn_guard.enabled,
                        "delayMs": cfn_guard.delay_ms,
                        "validateOnChange": cfn_guard.validate_on_change,
                        "enabledRulePacks": cfn_guard.enabled_rule_packs,
                        "timeout": cfn_guard.timeout,
                        "maxConcurrentValidations": cfn_guard.max_concurrent_validations,
                        "maxQueueSize": cfn_guard.max_queue_size,
                        "memoryCleanupInterval": cfn_guard.memory_cleanup_interval,
                        "maxMemoryUsage": cfn_guard.max_memory_usage,
                        "defaultSeverity": cfn_guard.default_severity,
                    }),
                    None => serde_json::Value::Null,
                },
            },
        });

        config
    }
}

impl AwsCloudformationExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "aws-cloudformation/cloudformation-languageserver",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "cloudformation-languageserver-{version}-{os}-{arch}-node22.zip",
            version = &release.version[1..],
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "win32",
            },
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x64",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| {
                eprintln!("no asset found matching: {:?}", asset_name);
                format!("no asset found matching {:?}", asset_name)
            })?;

        let extension_dir = env::current_dir()
            .unwrap()
            .join(".zed")
            .join("aws")
            .join(format!("cloudformation-languageserver-{}", release.version))
            .to_string_lossy()
            .to_string();

        fs::create_dir_all(&extension_dir)
            .map_err(|err| format!("failed to create directory '{}': {err}", extension_dir))
            .unwrap();

        let binary_path = format!("{}/cfn-lsp-server-standalone.js", extension_dir);

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &extension_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::Zip,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e} to path {binary_path}"))?;
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for AwsCloudformationExtension {
    fn new() -> Self {
        Self::new()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let raw_binary_path = self
            .language_server_binary_path(language_server_id, worktree)
            .unwrap_or_else(|e| {
                eprintln!("Error finding language server binary: {e}");
                e.to_string() // Fallback to default path
            })
            .to_string()
            .to_owned();

        let binary_path = PathBuf::from(&raw_binary_path);

        if !binary_path.exists() {
            return Err(format!(
                    "AWS Cloudformation language server not found at {:?}. Please run Zed's extension install process again.",
                    binary_path
                ));
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![raw_binary_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = self.load_aws_cloudformation_lsp_settings(worktree);
        let config = self.create_config(&settings);
        Ok(Some(config))
    }
}

zed_extension_api::register_extension!(AwsCloudformationExtension);
