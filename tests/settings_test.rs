use moon_monitor::settings::AppSettings;

#[test]
fn test_default_settings() {
    let settings = AppSettings::default();
    assert!(settings.dark_mode);
    assert_eq!(settings.update_interval_secs, 2);
    assert!(settings.animations_enabled);
    assert!(!settings.is_paused);
}

#[test]
fn test_settings_serde_roundtrip() {
    let settings = AppSettings {
        dark_mode: false,
        update_interval_secs: 5,
        animations_enabled: false,
        is_paused: true,
    };

    let json = serde_json::to_string(&settings).expect("Falha ao serializar");
    let loaded: AppSettings = serde_json::from_str(&json).expect("Falha ao desserializar");

    assert_eq!(loaded.dark_mode, false);
    assert_eq!(loaded.update_interval_secs, 5);
    assert_eq!(loaded.animations_enabled, false);
    assert_eq!(loaded.is_paused, true);
}
