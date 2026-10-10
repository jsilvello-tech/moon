use moon_monitor::utils::formatters::*;

#[test]
fn test_format_bytes_gib() {
    assert_eq!(format_bytes_gib(500), "500 B");
    assert_eq!(format_bytes_gib(1024), "1.0 KiB");
    assert_eq!(format_bytes_gib(1024 * 1024), "1.0 MiB");
    assert_eq!(format_bytes_gib(1024 * 1024 * 1024), "1.0 GiB");
    assert_eq!(format_bytes_gib(16 * 1024 * 1024 * 1024), "16.0 GiB");
}

#[test]
fn test_format_storage_bytes() {
    assert_eq!(format_storage_bytes(500), "500 B");
    assert_eq!(format_storage_bytes(1000), "1.0 KB");
    assert_eq!(format_storage_bytes(1_000_000), "1.0 MB");
    assert_eq!(format_storage_bytes(500_000_000_000), "500.0 GB");
}

#[test]
fn test_calculate_percentage() {
    assert_eq!(calculate_percentage(0, 0), 0.0);
    assert_eq!(calculate_percentage(50, 100), 50.0);
    assert_eq!(calculate_percentage(16, 32), 50.0);
    assert_eq!(calculate_percentage(100, 100), 100.0);
}

#[test]
fn test_format_uptime() {
    assert_eq!(format_uptime(45), "0m");
    assert_eq!(format_uptime(125), "2m");
    assert_eq!(format_uptime(3665), "1h 1m");
    assert_eq!(format_uptime(90060), "1d 1h 1m");
}

#[test]
fn test_format_frequency() {
    assert_eq!(format_frequency(0), "Não disponível");
    assert_eq!(format_frequency(800), "800 MHz");
    assert_eq!(format_frequency(2400), "2.40 GHz");
    assert_eq!(format_frequency(4200), "4.20 GHz");
}
