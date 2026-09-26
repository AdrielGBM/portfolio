use telar::{
    ColorScheme, RwSignal, detached, register_mode, set_mode, signal, system_preferences,
    use_color_scheme,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SchemePreference {
    #[default]
    System,
    Light,
    Dark,
}

impl SchemePreference {
    pub const ALL: [SchemePreference; 3] = [
        SchemePreference::System,
        SchemePreference::Light,
        SchemePreference::Dark,
    ];

    pub fn mode_id(self) -> &'static str {
        match self {
            SchemePreference::System => "system",
            SchemePreference::Light => "light",
            SchemePreference::Dark => "dark",
        }
    }

    pub fn resolve(self, system: Option<ColorScheme>) -> ColorScheme {
        match self {
            SchemePreference::Light => ColorScheme::Light,
            SchemePreference::Dark => ColorScheme::Dark,
            SchemePreference::System => system.unwrap_or(ColorScheme::Light),
        }
    }
}

thread_local! {
    static PREFERENCE: RwSignal<SchemePreference> = detached(|| signal(SchemePreference::default()));
}

pub fn register_scheme_modes() {
    for preference in SchemePreference::ALL {
        register_mode(preference.mode_id(), move || {
            PREFERENCE.with(|signal| signal.set(preference))
        });
    }
    set_mode(SchemePreference::default().mode_id());
}

// Routed through a theme mode rather than written to the signal directly, so Telar's hot-reload bridge carries the override across a reload.
pub fn set_scheme_preference(preference: SchemePreference) {
    set_mode(preference.mode_id());
}

pub fn use_scheme_preference() -> SchemePreference {
    PREFERENCE.with(|signal| signal.get())
}

pub fn use_scheme() -> ColorScheme {
    use_scheme_preference().resolve(use_color_scheme())
}

pub fn scheme_now() -> ColorScheme {
    PREFERENCE
        .with(|signal| signal.peek())
        .resolve(system_preferences().color_scheme)
}

#[cfg(test)]
#[path = "scheme_test.rs"]
mod tests;
