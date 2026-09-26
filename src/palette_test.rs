use telar::ColorScheme;

use super::{Contrast, Ramp, Step};

const SCHEMES: [ColorScheme; 2] = [ColorScheme::Light, ColorScheme::Dark];
const CONTRASTS: [Contrast; 2] = [Contrast::Standard, Contrast::High];

fn assert_close(actual: f32, expected: f32, tolerance: f32, what: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{what}: expected {expected}, got {actual}"
    );
}

#[test]
fn each_shade_keeps_its_seeds_chroma_and_hue() {
    for ramp in Ramp::ALL {
        let (_, seed_chroma, seed_hue, _) = ramp.seed().to_oklcha();
        let (_, chroma, hue, _) = ramp
            .shade(Step::S500, Contrast::Standard, ColorScheme::Dark)
            .to_oklcha();
        assert_close(chroma, seed_chroma, 0.002, &format!("{ramp:?} chroma"));
        assert_close(hue, seed_hue, 1.0, &format!("{ramp:?} hue"));
    }
}

#[test]
fn each_shade_sits_at_its_steps_lightness() {
    for ramp in Ramp::ALL {
        for scheme in SCHEMES {
            for step in [Step::S300, Step::S500, Step::S700] {
                let (lightness, _, _, _) = ramp.shade(step, Contrast::Standard, scheme).to_oklcha();
                assert_close(
                    lightness,
                    Contrast::Standard.lightness(step, scheme),
                    0.005,
                    &format!("{ramp:?} {} {scheme:?} lightness", step.number()),
                );
            }
        }
    }
}

#[test]
fn the_standard_dark_ramp_runs_from_095_to_005() {
    let lightness: Vec<f32> = Step::ALL
        .iter()
        .map(|step| Contrast::Standard.lightness(*step, ColorScheme::Dark))
        .collect();
    assert_eq!(
        lightness,
        [0.95, 0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1, 0.05]
    );
}

#[test]
fn the_light_scheme_mirrors_the_dark_one() {
    for contrast in CONTRASTS {
        for step in Step::ALL {
            assert_close(
                contrast.lightness(step, ColorScheme::Light),
                1.0 - contrast.lightness(step, ColorScheme::Dark),
                1e-6,
                &format!("{contrast:?} {}", step.number()),
            );
        }
    }
}

#[test]
fn step_50_is_the_ink_end_and_950_the_background_end() {
    for contrast in CONTRASTS {
        for scheme in SCHEMES {
            let lightness: Vec<f32> = Step::ALL
                .iter()
                .map(|step| contrast.lightness(*step, scheme))
                .collect();
            let monotonic = match scheme {
                ColorScheme::Dark => lightness.windows(2).all(|pair| pair[0] > pair[1]),
                ColorScheme::Light => lightness.windows(2).all(|pair| pair[0] < pair[1]),
            };
            assert!(monotonic, "{contrast:?} {scheme:?} ramp is not monotonic: {lightness:?}");
        }
    }
}

