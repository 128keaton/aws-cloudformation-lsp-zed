use serde_json::{Number, Value};
use std::path::PathBuf;
use std::{env, fs};

mod structs;
use structs::*;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

pub struct AwsCloudformationExtension {
    cached_binary_path: Option<String>,
}

fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}

impl AwsCloudformationExtension {
    fn get_default_lsp_init_options(
        &self,
        settings: Settings,
        version: String,
    ) -> InitializationOptions {
        InitializationOptions {
            feature_flags: Some(InitializationFeatureFlags {
                refresh_interval_ms: Some(Number::from(5 * 60 * 1000)),
                dynamic_refresh_interval_ms: Some(Number::from(60 * 1000)),
            }),
            settings: Some(settings),
            cloudformation: Some(CloudformationInitializationOptions {
                endpoint: Some("".to_string()),
            }),
            telemetry_enabled: Some(false),
            schema: Some(SchemaOptions {
                stale_days_threshold: Some(Number::from(5)),
            }),
            storage_dir: Some(".zed/aws/cloudformation-lsp".to_string()),
            log_level: Some(LogLevel::Info),
            client_info: Some(ClientInfo {
                client_id: Some("zed-aws-cloudformation-extension".to_string()),
                extension: Some(ClientInfoExtension {
                    name: Some("Zed - AWS Cloudformation LSP Extension".to_string()),
                    version: Some(version),
                }),
            }),
        }
    }

    fn get_default_lsp_settings(&self) -> Settings {
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
}

impl AwsCloudformationExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn get_base_config(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Value {
        return LspSettings::for_worktree(language_server_id.to_string().as_str(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_else(|| {
                eprintln!("No user settings found, using default settings.");
                serde_json::to_value(self.get_default_lsp_settings()).unwrap_or_else(|e| {
                    eprintln!("Failed to serialize default settings: {e}");
                    serde_json::Value::Null
                })
            });
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
                "cfn-lsp-server-standalone.js".to_string()
            })
            .to_string()
            .to_owned();

        let binary_path = PathBuf::from(&raw_binary_path);

        if !binary_path.exists() {
            return Err(format!(
                "AWS Cloudformation LSP binary not found at {:?}.",
                binary_path
            ));
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![raw_binary_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = self.get_base_config(language_server_id, worktree);
        let settings_struct: Settings = serde_json::from_value(settings).unwrap();

        let mut default_init_options = serde_json::to_value(
            self.get_default_lsp_init_options(settings_struct.clone(), "0.1.0".to_string()),
        )
        .unwrap();

        let init_options =
            LspSettings::for_worktree(language_server_id.to_string().as_str(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_else(|| {
                    eprintln!("No user init options found, using default init options.");
                    serde_json::to_value(
                        self.get_default_lsp_init_options(
                            settings_struct.clone(),
                            "0.1.0".to_string(),
                        ),
                    )
                    .unwrap_or_else(|e| {
                        eprintln!("Failed to serialize default init options: {e}");
                        serde_json::Value::Null
                    })
                });
        merge(&mut default_init_options, init_options);

        return Ok(Some(default_init_options));
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let mut default_settings = serde_json::to_value(self.get_default_lsp_settings()).unwrap();
        let settings = self.get_base_config(language_server_id, worktree);

        merge(&mut default_settings, settings);

        return Ok(Some(default_settings));
    }
}

zed_extension_api::register_extension!(AwsCloudformationExtension);
