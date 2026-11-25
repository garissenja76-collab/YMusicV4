//! AUTUMN's reusable visual tokens. Glass colors carry alpha; they do not blur content behind them.

use gpui_kit::*;

fn color(value: u32) -> Hsla {
    rgb(value).into()
}
pub const WINDOWS_FONT_FAMILIES: [&str; 2] = ["Inter", "Segoe UI"];
pub const DARK_ACCENT_HEX: &str = "5EAAFF";
pub const LIGHT_ACCENT_HEX: &str = "387FC3";

fn accent_color(hex: &str) -> Hsla {
    u32::from_str_radix(hex, 16).map(color).unwrap_or_else(|_| color(0x5EAAFF))
}

pub const TYPE: Typography = Typography {
    display: TextStyle { size: 30., line_height: 36. },
    h1: TextStyle { size: 24., line_height: 32. },
    h2: TextStyle { size: 20., line_height: 28. },
    body: TextStyle { size: 14., line_height: 20. },
    body_large: TextStyle { size: 16., line_height: 24. },
    label: TextStyle { size: 12., line_height: 16. },
    caption: TextStyle { size: 11., line_height: 14. },
};

pub const SPACE: Spacing = Spacing {
    s1: 4.,
    s2: 8.,
    s3: 12.,
    s4: 16.,
    s5: 20.,
    s6: 24.,
    s8: 32.,
};
pub const RADIUS: Radius = Radius { small: 8., control: 12., card: 16., dialog: 20. };
pub const BORDER: Border = Border { width: 1., focus: 2. };

#[derive(Clone, Copy)]
pub struct TextStyle {
    pub size: f32,
    pub line_height: f32,
}

#[derive(Clone, Copy)]
pub struct Typography {
    pub display: TextStyle,
    pub h1: TextStyle,
    pub h2: TextStyle,
    pub body: TextStyle,
    pub body_large: TextStyle,
    pub label: TextStyle,
    pub caption: TextStyle,
}

#[derive(Clone, Copy)]
pub struct Spacing {
    pub s1: f32,
    pub s2: f32,
    pub s3: f32,
    pub s4: f32,
    pub s5: f32,
    pub s6: f32,
    pub s8: f32,
}

#[derive(Clone, Copy)]
pub struct Radius {
    pub small: f32,
    pub control: f32,
    pub card: f32,
    pub dialog: f32,
}

#[derive(Clone, Copy)]
pub struct Border {
    pub width: f32,
    pub focus: f32,
}

#[derive(Clone, Copy)]
pub struct Shadow {
    pub offset_y: f32,
    pub blur: f32,
    pub alpha: f32,
}

impl Shadow {
    pub fn to_box_shadow(self) -> BoxShadow {
        BoxShadow::new(px(0.), px(self.offset_y), black().opacity(self.alpha))
            .blur_radius(px(self.blur))
    }
}

pub const SHADOW_CARD: Shadow = Shadow { offset_y: 4., blur: 16., alpha: 0.16 };
pub const SHADOW_PANEL: Shadow = Shadow { offset_y: 8., blur: 28., alpha: 0.25 };
pub const SHADOW_MODAL: Shadow = Shadow { offset_y: 18., blur: 48., alpha: 0.34 };

#[derive(Clone, Copy)]
pub struct Colors {
    pub primary: Hsla,
    pub secondary: Hsla,
    pub accent: Hsla,
    pub accent_hex: &'static str,
    pub background: Hsla,
    pub background_elevated: Hsla,
    pub glass_surface: Hsla,
    pub glass_surface_hover: Hsla,
    pub glass_border: Hsla,
    pub text_primary: Hsla,
    pub text_secondary: Hsla,
    pub text_muted: Hsla,
    pub success: Hsla,
    pub warning: Hsla,
    pub error: Hsla,
    pub player_surface: Hsla,
    pub glass_surface_opacity: f32,
    pub glass_hover_opacity: f32,
    pub glass_border_opacity: f32,
}

pub fn colors(dark: bool, amoled: bool) -> Colors {
    if dark {
        let background = if amoled { black() } else { rgb(0x0A1420).into() };
        let background_elevated = if amoled { black() } else { rgb(0x111E2B).into() };
        Colors {
            primary: color(0x7DB7FF),
            secondary: color(0xAFC4D8),
            accent: accent_color(DARK_ACCENT_HEX),
            accent_hex: DARK_ACCENT_HEX,
            background,
            background_elevated,
            glass_surface: color(0x142638).opacity(0.86),
            glass_surface_hover: color(0x1D3852).opacity(0.91),
            glass_border: color(0x8EC5FF).opacity(0.22),
            text_primary: color(0xF2F7FC),
            text_secondary: color(0xB5C6D7),
            text_muted: color(0x8295A8),
            success: color(0x43C58A),
            warning: color(0xFFBF69),
            error: color(0xFF727F),
            player_surface: color(0x08131F),
            glass_surface_opacity: 0.86,
            glass_hover_opacity: 0.91,
            glass_border_opacity: 0.22,
        }
    } else {
        Colors {
            primary: color(0x285B8F),
            secondary: color(0x54718F),
            accent: accent_color(LIGHT_ACCENT_HEX),
            accent_hex: LIGHT_ACCENT_HEX,
            background: color(0xEAF2FA),
            background_elevated: color(0xF6FAFF),
            glass_surface: color(0xFFFFFF).opacity(0.85),
            glass_surface_hover: color(0xEAF4FF).opacity(0.93),
            glass_border: color(0x477CAC).opacity(0.25),
            text_primary: color(0x142638),
            text_secondary: color(0x3E586F),
            text_muted: color(0x697F94),
            success: color(0x18794E),
            warning: color(0x955C00),
            error: color(0xB42335),
            player_surface: color(0x102338),
            glass_surface_opacity: 0.85,
            glass_hover_opacity: 0.93,
            glass_border_opacity: 0.25,
        }
    }
}
