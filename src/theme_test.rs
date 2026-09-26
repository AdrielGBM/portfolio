use std::cell::Cell;
use std::rc::Rc;

use telar::{
    Children, Color, ColorScheme, Container, LayoutStyle, Slots, SystemPreferences, effect,
    reset_layout_runtime, set_system_preferences, use_theme,
};

use super::{Act, ActPaletteProps, AppTheme, act_palette, install_theme, theme};
use crate::scheme::{SchemePreference, register_scheme_modes, set_scheme_preference};

const BODY_TEXT: f32 = 4.5;
const LARGE_TEXT: f32 = 3.0;

fn assert_aa(act: Act, scheme: ColorScheme) {
    let palette = AppTheme::for_act(act, scheme);
    let pairs = [
        ("ink / background", palette.ink, palette.background, BODY_TEXT),
        ("ink / surface", palette.ink, palette.surface, BODY_TEXT),
        ("ink_muted / background", palette.ink_muted, palette.background, BODY_TEXT),
        ("ink_muted / surface", palette.ink_muted, palette.surface, BODY_TEXT),
        ("accent_ink / accent", palette.accent_ink, palette.accent, BODY_TEXT),
        ("ink_display / background", palette.ink_display, palette.background, LARGE_TEXT),
    ];
    let failures: Vec<String> = pairs
        .iter()
        .filter_map(|(pair, foreground, background, minimum)| {
            let ratio = foreground.contrast_ratio(*background);
            (ratio < *minimum).then(|| format!("{pair} is {ratio:.2}:1, needs {minimum}:1"))
        })
        .collect();
    assert!(
        failures.is_empty(),
        "{act:?} in {scheme:?} fails WCAG AA: {failures:#?}"
    );
}

#[test]
fn opening_light_meets_aa() {
    assert_aa(Act::Opening, ColorScheme::Light);
}

#[test]
fn opening_dark_meets_aa() {
    assert_aa(Act::Opening, ColorScheme::Dark);
}

#[test]
fn act_one_light_meets_aa() {
    assert_aa(Act::One, ColorScheme::Light);
}

#[test]
fn act_one_dark_meets_aa() {
    assert_aa(Act::One, ColorScheme::Dark);
}

#[test]
fn act_two_light_meets_aa() {
    assert_aa(Act::Two, ColorScheme::Light);
}

#[test]
fn act_two_dark_meets_aa() {
    assert_aa(Act::Two, ColorScheme::Dark);
}

#[test]
fn act_three_light_meets_aa() {
    assert_aa(Act::Three, ColorScheme::Light);
}

#[test]
fn act_three_dark_meets_aa() {
    assert_aa(Act::Three, ColorScheme::Dark);
}

#[test]
fn credits_light_meets_aa() {
    assert_aa(Act::Credits, ColorScheme::Light);
}

#[test]
fn credits_dark_meets_aa() {
    assert_aa(Act::Credits, ColorScheme::Dark);
}

#[test]
fn each_act_draws_from_the_ramp_the_storyboard_gives_it() {
    use crate::palette::{Contrast, Ramp};
    let expected = [
        (Act::Opening, Ramp::Neutral, Contrast::Standard),
        (Act::One, Ramp::Primary, Contrast::Standard),
        (Act::Two, Ramp::Neutral, Contrast::High),
        (Act::Three, Ramp::Secondary, Contrast::Standard),
        (Act::Credits, Ramp::Neutral, Contrast::Standard),
    ];
    for (act, ramp, contrast) in expected {
        let steps = act.steps();
        assert_eq!((steps.ramp, steps.contrast), (ramp, contrast), "{act:?}");
    }
}

fn system_scheme(scheme: ColorScheme) {
    set_system_preferences(SystemPreferences {
        color_scheme: Some(scheme),
        ..SystemPreferences::default()
    });
}

fn recording_child(seen: Rc<Cell<Option<Color>>>) -> Children {
    Children::new(move || {
        let seen = Rc::clone(&seen);
        effect(move || seen.set(Some(use_theme::<AppTheme>().accent)));
        let mut slots = Slots::new();
        slots.push(None, Box::new(Container::new(LayoutStyle::new(), vec![])?));
        Ok(slots)
    })
}

#[test]
fn the_base_theme_follows_the_system_until_overridden() {
    reset_layout_runtime();
    system_scheme(ColorScheme::Light);
    install_theme();
    assert_eq!(theme(), AppTheme::base(ColorScheme::Light));

    system_scheme(ColorScheme::Dark);
    assert_eq!(theme(), AppTheme::base(ColorScheme::Dark));

    set_scheme_preference(SchemePreference::Light);
    assert_eq!(theme(), AppTheme::base(ColorScheme::Light));

    system_scheme(ColorScheme::Light);
    system_scheme(ColorScheme::Dark);
    assert_eq!(
        theme(),
        AppTheme::base(ColorScheme::Light),
        "a manual override survives a change of the system scheme"
    );

    set_scheme_preference(SchemePreference::System);
    assert_eq!(theme(), AppTheme::base(ColorScheme::Dark));
}

#[test]
fn an_act_palette_scopes_its_theme_and_follows_the_scheme() {
    reset_layout_runtime();
    system_scheme(ColorScheme::Light);
    register_scheme_modes();
    let seen = Rc::new(Cell::new(None));
    let _scene = act_palette(
        ActPaletteProps::props().act(Act::One).build(),
        recording_child(Rc::clone(&seen)),
    )
    .expect("the act palette builds");
    assert_eq!(
        seen.get(),
        Some(AppTheme::for_act(Act::One, ColorScheme::Light).accent)
    );

    set_scheme_preference(SchemePreference::Dark);
    assert_eq!(
        seen.get(),
        Some(AppTheme::for_act(Act::One, ColorScheme::Dark).accent)
    );

    set_scheme_preference(SchemePreference::System);
    system_scheme(ColorScheme::Dark);
    system_scheme(ColorScheme::Light);
    assert_eq!(
        seen.get(),
        Some(AppTheme::for_act(Act::One, ColorScheme::Light).accent)
    );
}

#[test]
fn an_act_palette_wraps_exactly_one_child() {
    reset_layout_runtime();
    let two_children = Children::new(|| {
        let mut slots = Slots::new();
        for _ in 0..2 {
            slots.push(None, Box::new(Container::new(LayoutStyle::new(), vec![])?));
        }
        Ok(slots)
    });
    let built = act_palette(ActPaletteProps::props().act(Act::Two).build(), two_children);
    assert!(built.is_err(), "two children under one act palette are refused");
}
