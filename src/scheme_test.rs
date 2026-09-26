use telar::ColorScheme;

use super::SchemePreference;

#[test]
fn system_follows_the_reported_scheme() {
    for scheme in [ColorScheme::Light, ColorScheme::Dark] {
        assert_eq!(SchemePreference::System.resolve(Some(scheme)), scheme);
    }
}

#[test]
fn an_unknown_system_scheme_reads_as_light() {
    assert_eq!(SchemePreference::System.resolve(None), ColorScheme::Light);
}

#[test]
fn a_manual_choice_ignores_the_system() {
    for system in [None, Some(ColorScheme::Light), Some(ColorScheme::Dark)] {
        assert_eq!(SchemePreference::Light.resolve(system), ColorScheme::Light);
        assert_eq!(SchemePreference::Dark.resolve(system), ColorScheme::Dark);
    }
}

#[test]
fn each_preference_has_its_own_mode_id() {
    let mut ids: Vec<&str> = SchemePreference::ALL.iter().map(|p| p.mode_id()).collect();
    ids.dedup();
    assert_eq!(ids.len(), SchemePreference::ALL.len());
}
