use std::cell::Cell;

use telar::{
    Children, Color, ColorScheme, LayoutError, LayoutItem, OwnerId, ScopedTheme, ThemeTokens,
    detached, dispose_owner, effect, owner_scope, provide_theme, set_theme, use_theme,
};

use crate::palette::{Contrast, Ramp, Step};
use crate::scheme::{register_scheme_modes, scheme_now, use_scheme};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    Opening,
    One,
    Two,
    Three,
    Credits,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActSteps {
    pub ramp: Ramp,
    pub contrast: Contrast,
    pub background: Step,
    pub surface: Step,
    pub ink: Step,
    pub ink_muted: Step,
    pub ink_display: Step,
    pub accent: Step,
    pub accent_ink: Step,
    pub rule: Step,
}

impl Act {
    pub const ALL: [Act; 5] = [Act::Opening, Act::One, Act::Two, Act::Three, Act::Credits];

    pub fn steps(self) -> ActSteps {
        match self {
            Act::Opening => ActSteps {
                ramp: Ramp::Neutral,
                contrast: Contrast::Standard,
                background: Step::S950,
                surface: Step::S900,
                ink: Step::S50,
                ink_muted: Step::S400,
                ink_display: Step::S50,
                accent: Step::S200,
                accent_ink: Step::S950,
                rule: Step::S700,
            },
            Act::One => ActSteps {
                ramp: Ramp::Primary,
                contrast: Contrast::Standard,
                background: Step::S950,
                surface: Step::S900,
                ink: Step::S50,
                ink_muted: Step::S400,
                ink_display: Step::S200,
                accent: Step::S300,
                accent_ink: Step::S950,
                rule: Step::S700,
            },
            Act::Two => ActSteps {
                ramp: Ramp::Neutral,
                contrast: Contrast::High,
                background: Step::S950,
                surface: Step::S900,
                ink: Step::S50,
                ink_muted: Step::S300,
                ink_display: Step::S50,
                accent: Step::S50,
                accent_ink: Step::S950,
                rule: Step::S600,
            },
            Act::Three => ActSteps {
                ramp: Ramp::Secondary,
                contrast: Contrast::Standard,
                background: Step::S950,
                surface: Step::S900,
                ink: Step::S50,
                ink_muted: Step::S400,
                ink_display: Step::S200,
                accent: Step::S300,
                accent_ink: Step::S950,
                rule: Step::S700,
            },
            Act::Credits => ActSteps {
                ramp: Ramp::Neutral,
                contrast: Contrast::Standard,
                background: Step::S950,
                surface: Step::S900,
                ink: Step::S50,
                ink_muted: Step::S300,
                ink_display: Step::S100,
                accent: Step::S200,
                accent_ink: Step::S950,
                rule: Step::S700,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, ThemeTokens)]
#[theme(
    surface_alt = self.background,
    scrollbar = self.ink_muted.with_alpha(0.55),
    highlight_low = self.ink.with_alpha(0.06),
    highlight_med = self.ink.with_alpha(0.12),
    highlight_high = self.ink.with_alpha(0.2)
)]
#[theme(default(radius, spacing, icon_size, success, warning, error, info))]
pub struct AppTheme {
    pub background: Color,
    pub surface: Color,
    pub ink: Color,
    #[token(muted)]
    pub ink_muted: Color,
    pub ink_display: Color,
    #[token(primary)]
    pub accent: Color,
    #[token(on_primary)]
    pub accent_ink: Color,
    #[token(border)]
    pub rule: Color,
}

impl AppTheme {
    pub fn for_act(act: Act, scheme: ColorScheme) -> Self {
        let steps = act.steps();
        let shade = |step| steps.ramp.shade(step, steps.contrast, scheme);
        Self {
            background: shade(steps.background),
            surface: shade(steps.surface),
            ink: shade(steps.ink),
            ink_muted: shade(steps.ink_muted),
            ink_display: shade(steps.ink_display),
            accent: shade(steps.accent),
            accent_ink: shade(steps.accent_ink),
            rule: shade(steps.rule),
        }
    }

    pub fn base(scheme: ColorScheme) -> Self {
        Self::for_act(Act::Opening, scheme)
    }
}

pub fn theme() -> AppTheme {
    use_theme::<AppTheme>()
}

thread_local! {
    static BASE_FOLLOWER: Cell<Option<OwnerId>> = const { Cell::new(None) };
}

pub fn install_theme() {
    register_scheme_modes();
    if let Some(previous) = BASE_FOLLOWER.take() {
        dispose_owner(previous);
    }
    let scope = detached(owner_scope);
    effect(|| set_theme(AppTheme::base(use_scheme())));
    BASE_FOLLOWER.set(Some(scope.id()));
}

#[telar::component]
pub fn act_palette(act: Act, children: Children) -> Result<Box<dyn LayoutItem>, LayoutError> {
    let palette = ScopedTheme::new(AppTheme::for_act(act, scheme_now()));
    effect(move || palette.set(AppTheme::for_act(act, use_scheme())));
    let provider = provide_theme(palette, move || {
        let built = children.build()?.take_default();
        let count = built.len();
        let [only]: [Box<dyn LayoutItem>; 1] = built.try_into().map_err(|_| {
            LayoutError::Engine(format!("act_palette wraps exactly one child, got {count}"))
        })?;
        Ok(only)
    })?;
    Ok(Box::new(provider))
}

#[cfg(test)]
#[path = "theme_test.rs"]
mod tests;
