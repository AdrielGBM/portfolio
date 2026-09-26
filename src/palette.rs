use telar::{Color, ColorScheme};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ramp {
    Neutral,
    Primary,
    Secondary,
}

impl Ramp {
    pub const ALL: [Ramp; 3] = [Ramp::Neutral, Ramp::Primary, Ramp::Secondary];

    fn seed_hex(self) -> &'static str {
        match self {
            Ramp::Neutral => "#a5a9ae",
            Ramp::Primary => "#76ae95",
            Ramp::Secondary => "#76abae",
        }
    }

    pub fn seed(self) -> Color {
        Color::from_hex(self.seed_hex()).expect("ramp seeds are valid hex literals")
    }

    pub fn shade(self, step: Step, contrast: Contrast, scheme: ColorScheme) -> Color {
        let (_, chroma, hue, _) = self.seed().to_oklcha();
        Color::from_oklch(contrast.lightness(step, scheme), chroma, hue)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    S50,
    S100,
    S200,
    S300,
    S400,
    S500,
    S600,
    S700,
    S800,
    S900,
    S950,
}

impl Step {
    pub const ALL: [Step; 11] = [
        Step::S50,
        Step::S100,
        Step::S200,
        Step::S300,
        Step::S400,
        Step::S500,
        Step::S600,
        Step::S700,
        Step::S800,
        Step::S900,
        Step::S950,
    ];

    pub fn number(self) -> u16 {
        match self {
            Step::S50 => 50,
            Step::S100 => 100,
            Step::S200 => 200,
            Step::S300 => 300,
            Step::S400 => 400,
            Step::S500 => 500,
            Step::S600 => 600,
            Step::S700 => 700,
            Step::S800 => 800,
            Step::S900 => 900,
            Step::S950 => 950,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contrast {
    Standard,
    High,
}

impl Contrast {
    // Written for the dark scheme, where step 50 is the ink end; the light scheme mirrors it so 950 stays the background end in both.
    fn dark_lightness(self) -> [f32; 11] {
        match self {
            Contrast::Standard => [0.95, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.05],
            Contrast::High => [1.0, 0.95, 0.85, 0.75, 0.65, 0.45, 0.35, 0.25, 0.15, 0.05, 0.0],
        }
    }

    pub fn lightness(self, step: Step, scheme: ColorScheme) -> f32 {
        let dark = self.dark_lightness()[step as usize];
        match scheme {
            ColorScheme::Dark => dark,
            ColorScheme::Light => 1.0 - dark,
        }
    }
}

#[cfg(test)]
#[path = "palette_test.rs"]
mod tests;
