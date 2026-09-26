use super::{TypeScale, display_size};

const WIDTHS: [f32; 6] = [320.0, 390.0, 768.0, 1280.0, 1440.0, 2560.0];

#[test]
fn the_display_size_is_between_20_and_24_percent_of_the_width() {
    for width in WIDTHS {
        let fraction = display_size(width) / width;
        assert!(
            (0.20..=0.24).contains(&fraction),
            "display is {fraction} of a {width}px surface"
        );
    }
}

#[test]
fn each_scale_is_ordered_from_headline_down_to_caption() {
    for width in WIDTHS {
        let scale = TypeScale::at(width);
        let sizes = [scale.headline, scale.title, scale.body, scale.mono, scale.caption];
        assert!(
            sizes.windows(2).all(|pair| pair[0] > pair[1]),
            "{width}px: {scale:?}"
        );
    }
}

#[test]
fn the_scale_grows_with_the_surface() {
    for pair in WIDTHS.windows(2) {
        let (narrow, wide) = (TypeScale::at(pair[0]), TypeScale::at(pair[1]));
        let grows = |size: fn(&TypeScale) -> f32| size(&wide) >= size(&narrow);
        assert!(
            grows(|s| s.headline)
                && grows(|s| s.title)
                && grows(|s| s.body)
                && grows(|s| s.mono)
                && grows(|s| s.caption),
            "{}px {narrow:?} vs {}px {wide:?}",
            pair[0],
            pair[1]
        );
    }
}

#[test]
fn body_text_never_drops_below_16px() {
    for width in WIDTHS {
        assert!(TypeScale::at(width).body >= 16.0, "{width}px");
    }
}
