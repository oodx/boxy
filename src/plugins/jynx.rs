// ================ JYNX INTEGRATION SYSTEM ================
// Enhanced output formatting with jynx integration for beautiful CLI experience
// Falls back gracefully when jynx is not available

use std::io::Write;
use std::process::Command;

/// Enhanced print function with jynx integration
pub fn jynx_println(content: &str, template: &str, jynx: &JynxPlugin) {
    let enhanced_content = pipe_to_jynx(content, template, jynx);
    print!("{}", enhanced_content);
}

/// Enhanced output with jynx integration
pub fn pipe_to_jynx(content: &str, template: &str, jynx: &JynxPlugin) -> String {
    // Return original content if jynx integration is not active
    if !jynx.is_active() {
        return content.to_string();
    }

    // Use jynx for enhanced formatting
    let mut cmd = Command::new("jynx");

    // Configure jynx command based on template type
    match template {
        "help" => {
            cmd.args(&["--template", "help", "--style", "enhanced"]);
        }
        "list" => {
            cmd.args(&["--template", "list", "--bullets", "→"]);
        }
        "success" => {
            cmd.args(&["--template", "success", "--icon", "✅"]);
        }
        "error" => {
            cmd.args(&["--template", "error", "--icon", "❌"]);
        }
        "warning" => {
            cmd.args(&["--template", "warning", "--icon", "⚠️"]);
        }
        "info" => {
            cmd.args(&["--template", "info", "--icon", "ℹ️"]);
        }
        "migration" => {
            cmd.args(&["--template", "guide", "--style", "migration"]);
        }
        "theme_list" => {
            cmd.args(&["--template", "themes", "--format", "compact"]);
        }
        _ => {
            // Generic enhancement
            cmd.args(&["--enhance"]);
        }
    }

    // Execute jynx with content as stdin
    match cmd
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
    {
        Ok(mut child) => {
            // Send content to jynx stdin
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = stdin.write_all(content.as_bytes());
            }

            // Get jynx output
            match child.wait_with_output() {
                Ok(output) => {
                    if output.status.success() {
                        String::from_utf8_lossy(&output.stdout).to_string()
                    } else {
                        // Fallback to original content if jynx fails
                        content.to_string()
                    }
                }
                Err(_) => content.to_string(),
            }
        }
        Err(_) => content.to_string(),
    }
}

/// Jynx availability detection with version checking
pub struct JynxPlugin {
    available: bool,
    version: Option<String>,
    supports_templates: bool,
    no_color_requested: bool,
}

impl JynxPlugin {
    /// Get formatted jynx version for display
    pub fn get_version_string(&self) -> String {
        match &self.version {
            Some(version) => format!("with jynx {}", version),
            None => "jynx not detected".to_string(),
        }
    }
}

impl JynxPlugin {
    /// Initialize jynx integration with comprehensive detection
    pub fn new(no_color: bool) -> Self {
        let mut integration = JynxPlugin {
            available: false,
            version: None,
            supports_templates: false,
            no_color_requested: no_color,
        };

        // Skip jynx detection if --no-color is explicitly requested
        if no_color {
            return integration;
        }

        // Check if jynx is available in PATH
        if let Ok(output) = Command::new("jynx").arg("--version").output() {
            if output.status.success() {
                if let Ok(version_output) = String::from_utf8(output.stdout) {
                    integration.available = true;
                    integration.version = Some(version_output.trim().to_string());

                    // Check for template support (jynx 0.3.0+)
                    integration.supports_templates = version_output.contains("0.3")
                        || version_output.contains("0.4")
                        || version_output.contains("0.5")
                        || version_output.contains("1.")
                        || version_output.contains("2.");
                }
            }
        }

        integration
    }

    /// Check if jynx is available and color is enabled
    pub fn is_active(&self) -> bool {
        self.available && !self.no_color_requested
    }
}
