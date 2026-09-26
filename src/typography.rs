use telar::{Breakpoints, Memo, breakpoint, memo, use_surface_width};

pub const DISPLAY_WIDTH_FRACTION: f32 = 0.22;

pub fn display_size(surface_width: f32) -> f32 {
    surface_width * DISPLAY_WIDTH_FRACTION
}

pub fn follow_display_size() -> Memo<f32> {
    memo(|| display_size(use_surface_width()))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeScale {
    pub headline: f32,
    pub title: f32,
    pub body: f32,
    pub mono: f32,
    pub caption: f32,
}

impl TypeScale {
    pub fn modular(body: f32, ratio: f32) -> Self {
        Self {
            headline: body * ratio.powi(4),
            title: body * ratio.powi(2),
            body,
            mono: body / ratio.sqrt(),
            caption: body / ratio,
        }
    }

    fn breakpoints() -> Breakpoints<TypeScale> {
        breakpoint(Self::modular(16.0, 1.2))
            .at(640.0, Self::modular(17.0, 1.25))
            .at(1024.0, Self::modular(18.0, 1.3))
            .at(1440.0, Self::modular(20.0, 1.333))
    }

    pub fn at(surface_width: f32) -> Self {
        *Self::breakpoints().value_at(surface_width)
    }

    pub fn follow() -> Memo<TypeScale> {
        Self::breakpoints().follow()
    }
}

#[cfg(test)]
#[path = "typography_test.rs"]
mod tests;
