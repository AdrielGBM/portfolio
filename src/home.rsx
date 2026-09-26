[logic]
use crate::typography::TypeScale;

let clicks = signal(0i32);
let type_scale = TypeScale::follow();

[view]
col grow:1 gap:16 pad:32 align:center fill:$theme.background
    text "Hello from Telar" font_size:$type_scale.title color:$theme.ink
    text "Clicks · {$clicks}" font_size:$type_scale.body color:$theme.ink_muted
    row gap:10
        button label:"+1" fill:$theme.accent on_press:(|| $clicks += 1)
        button label:"Reset" ghost on_press:(|| $clicks.set(0))

[preview "Home"]
home
