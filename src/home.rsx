[logic]
let clicks = signal(0i32);

[view]
col grow:1 gap:16 pad:32 align:center fill:$theme.surface_alt
    text "Hello from Telar" font_size:32 color:$theme.ink
    text "Clicks · {$clicks}" font_size:16 color:$theme.muted
    row gap:10
        button label:"+1" fill:$theme.primary on_press:(|| $clicks += 1)
        button label:"Reset" ghost on_press:(|| $clicks.set(0))

[preview "Home"]
home
