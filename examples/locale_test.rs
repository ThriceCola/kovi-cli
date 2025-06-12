//! AI Generation
//!
//! Example program demonstrating locale detection functionality
//!
//! This example shows how the kovi-cli detects system locale on different platforms,
//! with special support for Windows systems.
//!
//! Run with: `cargo run --example locale_test`

use std::env;

fn main() {
    println!("=== Kovi CLI Locale Detection Demo ===\n");

    // Show current system locale detection
    println!("🌍 Detecting system locale...");
    let detected_locale = get_system_locale();
    println!("   Detected locale: {}", detected_locale);

    // Show environment variables that affect locale detection
    println!("\n📋 Environment variables:");
    if let Ok(lc_all) = env::var("LC_ALL") {
        println!("   LC_ALL: {}", lc_all);
    } else {
        println!("   LC_ALL: (not set)");
    }

    if let Ok(lang) = env::var("LANG") {
        println!("   LANG: {}", lang);
    } else {
        println!("   LANG: (not set)");
    }

    // Show platform-specific information
    println!("\n🖥️  Platform information:");
    println!("   Operating System: {}", env::consts::OS);
    println!("   Architecture: {}", env::consts::ARCH);

    #[cfg(target_os = "windows")]
    {
        println!("   Windows locale detection: ENABLED");
        println!("   - PowerShell method available");
        println!("   - WMIC fallback available");
        println!("   - Locale code conversion supported");

        // Test Windows locale detection
        test_windows_locale_detection();
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("   Unix-style locale detection: ENABLED");
        println!("   - Using LC_ALL/LANG environment variables");
    }

    // Show supported locales
    println!("\n🗣️  Supported locales:");
    println!("   - English (en-US) - Default");
    println!("   - Chinese Simplified (zh-CN)");

    #[cfg(target_os = "windows")]
    {
        println!("\n🪟 Windows locale codes supported:");
        let test_codes = vec![
            ("0804", "Chinese Simplified"),
            ("0404", "Chinese Traditional"),
            ("0409", "English (US)"),
            ("0809", "English (UK)"),
            ("0411", "Japanese"),
            ("0412", "Korean"),
            ("040c", "French"),
            ("0407", "German"),
        ];

        for (code, desc) in test_codes {
            println!(
                "   - {} → {} ({})",
                code,
                convert_windows_locale_code(code),
                desc
            );
        }
    }

    // Instructions for testing
    println!("\n🧪 Testing locale detection:");
    println!("   To test different locales, try:");

    #[cfg(target_os = "windows")]
    {
        println!("   Windows:");
        println!("     - Change system locale in Windows Settings");
        println!("     - Or use PowerShell: Get-Culture");
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("   Unix/Linux/macOS:");
        println!("     - LC_ALL=zh-CN cargo run --example locale_test");
        println!("     - LANG=en-US cargo run --example locale_test");
    }

    println!("\n✅ Locale detection demo completed!");
}

fn get_system_locale() -> String {
    // 首先尝试标准的 Unix 环境变量
    if let Ok(locale) = env::var("LC_ALL") {
        return locale;
    }

    if let Ok(locale) = env::var("LANG") {
        return locale;
    }

    // Windows 系统特有的处理
    #[cfg(target_os = "windows")]
    {
        if let Some(locale) = get_windows_locale() {
            return locale;
        }
    }

    "en-US".to_string()
}

#[cfg(target_os = "windows")]
fn get_windows_locale() -> Option<String> {
    // 尝试使用 PowerShell 获取系统语言
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-Culture | Select-Object -ExpandProperty Name",
        ])
        .output()
    {
        if output.status.success() {
            let locale_str = String::from_utf8_lossy(&output.stdout);
            let locale_str = locale_str.trim();
            if !locale_str.is_empty() {
                return Some(locale_str.to_string());
            }
        }
    }

    // 如果 PowerShell 失败，尝试使用 wmic
    if let Ok(output) = Command::new("wmic")
        .args(&["os", "get", "locale", "/value"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("Locale=") {
                    let locale_code = line.strip_prefix("Locale=").unwrap_or("").trim();
                    if !locale_code.is_empty() {
                        // 将 Windows 语言代码转换为标准格式
                        return Some(convert_windows_locale_code(locale_code));
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn convert_windows_locale_code(code: &str) -> String {
    // 将常见的 Windows 语言代码转换为标准格式
    match code {
        "0804" => "zh-CN".to_string(), // 中文(简体)
        "0404" => "zh-TW".to_string(), // 中文(繁体)
        "0409" => "en-US".to_string(), // 英语(美国)
        "0809" => "en-GB".to_string(), // 英语(英国)
        "0411" => "ja-JP".to_string(), // 日语
        "0412" => "ko-KR".to_string(), // 韩语
        "040c" => "fr-FR".to_string(), // 法语
        "0407" => "de-DE".to_string(), // 德语
        "0410" => "it-IT".to_string(), // 意大利语
        "0c0a" => "es-ES".to_string(), // 西班牙语
        "0416" => "pt-BR".to_string(), // 葡萄牙语(巴西)
        "0419" => "ru-RU".to_string(), // 俄语
        _ => {
            // 如果是未知代码，返回默认值
            "en-US".to_string()
        }
    }
}

#[cfg(target_os = "windows")]
fn test_windows_locale_detection() {
    println!("\n🧪 Testing Windows locale detection methods:");

    // Test PowerShell method
    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-NoProfile",
            "-Command",
            "Get-Culture | Select-Object -ExpandProperty Name",
        ])
        .output()
    {
        if output.status.success() {
            let locale = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!(
                "   PowerShell method: {}",
                if locale.is_empty() {
                    "(empty)"
                } else {
                    &locale
                }
            );
        } else {
            println!("   PowerShell method: Failed");
        }
    } else {
        println!("   PowerShell method: Not available");
    }

    // Test wmic method
    if let Ok(output) = Command::new("wmic")
        .args(&["os", "get", "locale", "/value"])
        .output()
    {
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut found_locale = false;
            for line in output_str.lines() {
                if line.starts_with("Locale=") {
                    let locale_code = line.strip_prefix("Locale=").unwrap_or("").trim();
                    if !locale_code.is_empty() {
                        let converted = convert_windows_locale_code(locale_code);
                        println!("   WMIC method: {} → {}", locale_code, converted);
                        found_locale = true;
                        break;
                    }
                }
            }
            if !found_locale {
                println!("   WMIC method: No locale found");
            }
        } else {
            println!("   WMIC method: Failed");
        }
    } else {
        println!("   WMIC method: Not available");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_system_locale() {
        let locale = get_system_locale();
        assert!(!locale.is_empty());
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_convert_windows_locale_code() {
        assert_eq!(convert_windows_locale_code("0804"), "zh-CN");
        assert_eq!(convert_windows_locale_code("0409"), "en-US");
        assert_eq!(convert_windows_locale_code("9999"), "en-US");
    }
}
