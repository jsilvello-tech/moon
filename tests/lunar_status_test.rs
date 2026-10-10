use moon_monitor::models::lunar_status::LunarPhase;

#[test]
fn test_lunar_phase_thresholds() {
    // Crescent (< 30%)
    assert_eq!(LunarPhase::from_cpu_usage(0.0), LunarPhase::Crescent);
    assert_eq!(LunarPhase::from_cpu_usage(15.5), LunarPhase::Crescent);
    assert_eq!(LunarPhase::from_cpu_usage(29.9), LunarPhase::Crescent);

    // Gibbous (30% - 69%)
    assert_eq!(LunarPhase::from_cpu_usage(30.0), LunarPhase::Gibbous);
    assert_eq!(LunarPhase::from_cpu_usage(50.0), LunarPhase::Gibbous);
    assert_eq!(LunarPhase::from_cpu_usage(69.9), LunarPhase::Gibbous);

    // Full (70% - 89%)
    assert_eq!(LunarPhase::from_cpu_usage(70.0), LunarPhase::Full);
    assert_eq!(LunarPhase::from_cpu_usage(80.0), LunarPhase::Full);
    assert_eq!(LunarPhase::from_cpu_usage(89.9), LunarPhase::Full);

    // Eclipse (>= 90%)
    assert_eq!(LunarPhase::from_cpu_usage(90.0), LunarPhase::Eclipse);
    assert_eq!(LunarPhase::from_cpu_usage(95.0), LunarPhase::Eclipse);
    assert_eq!(LunarPhase::from_cpu_usage(100.0), LunarPhase::Eclipse);
}

#[test]
fn test_lunar_phase_codes() {
    assert_eq!(LunarPhase::Crescent.phase_code(), 0);
    assert_eq!(LunarPhase::Gibbous.phase_code(), 1);
    assert_eq!(LunarPhase::Full.phase_code(), 2);
    assert_eq!(LunarPhase::Eclipse.phase_code(), 3);
}
