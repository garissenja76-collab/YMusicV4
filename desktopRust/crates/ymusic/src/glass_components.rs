//! Small GPUI builders that apply AUTUMN's visual tokens to the existing component APIs.

use gpui_kit::component::button::{Button, ButtonCustomVariant, ButtonRounded, ButtonVariants};
use gpui_kit::component::dialog::Dialog;
use gpui_kit::component::menu::PopupMenuStyle;
use gpui_kit::component::{ActiveTheme, Colorize, FocusableExt, Sizable, StyledExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::design_system::{self, BORDER, RADIUS, SHADOW_CARD, SHADOW_MODAL, SHADOW_PANEL};

pub struct GlassCard;

impl GlassCard {
    pub fn surface(element: Div, cx: &App) -> Div {
        let colors = design_system::active_colors(cx);
        element
            .rounded(px(RADIUS.card))
            .border(px(BORDER.width))
            .border_color(colors.glass_border)
            .bg(colors.glass_surface)
            .shadow(vec![SHADOW_CARD.to_box_shadow()])
    }

    pub fn wrap(child: impl IntoElement, cx: &App) -> Stateful<Div> {
        let colors = design_system::active_colors(cx);
        Self::surface(div(), cx)
            .hover(move |style| style.bg(colors.glass_surface_hover))
            .active(move |style| style.bg(colors.glass_surface_hover))
            .child(child)
    }

    pub fn interactive(element: Stateful<Div>, cx: &App) -> Stateful<Div> {
        let colors = design_system::active_colors(cx);
        element
            .rounded(px(RADIUS.card))
            .border(px(BORDER.width))
            .border_color(colors.glass_border)
            .bg(colors.glass_surface)
            .shadow(vec![SHADOW_CARD.to_box_shadow()])
    }
}

pub struct GlassButton;

impl GlassButton {
    pub fn primary(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
        Button::new(id)
            .label(label)
            .primary()
            .with_size(Size::Large)
            .h(px(40.))
            .rounded(ButtonRounded::Size(px(RADIUS.control)))
            .focus_ring(true)
    }

    pub fn secondary(id: impl Into<ElementId>, label: impl Into<SharedString>, cx: &App) -> Button {
        let colors = design_system::active_colors(cx);
        Button::new(id)
            .label(label)
            .custom(
                ButtonCustomVariant::new(cx)
                    .color(colors.glass_surface)
                    .foreground(cx.theme().foreground)
                    .hover(colors.glass_surface_hover)
                    .active(colors.glass_surface_hover),
            )
            .with_size(Size::Large)
            .h(px(40.))
            .rounded(ButtonRounded::Size(px(RADIUS.control)))
            .border(px(BORDER.width))
            .border_color(colors.glass_border)
            .focus_ring(true)
    }

    pub fn ghost(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Button {
        Button::new(id)
            .label(label)
            .ghost()
            .with_size(Size::Large)
            .h(px(40.))
            .rounded(ButtonRounded::Size(px(RADIUS.control)))
            .focus_ring(true)
    }

    pub fn icon(
        id: impl Into<ElementId>,
        icon: impl Into<Icon>,
        accessible_name: impl Into<SharedString>,
    ) -> Button {
        Button::new(id)
            .icon(icon.into())
            .accessibility_label(accessible_name)
            .ghost()
            .with_size(Size::Medium)
            .size(px(40.))
            .rounded(ButtonRounded::Size(px(RADIUS.control)))
            .focus_ring(true)
    }
}

pub struct GlassSidebar;

impl GlassSidebar {
    pub fn wrap(element: Div, cx: &App) -> Div {
        let colors = design_system::active_colors(cx);
        element
            .bg(colors.glass_surface)
            .border(px(BORDER.width))
            .border_color(colors.glass_border)
    }
}

pub struct GlassNavigationItem;

impl GlassNavigationItem {
    pub fn wrap(element: Div, selected: bool, selected_fill: Hsla, cx: &App) -> Stateful<Div> {
        let hover = design_system::active_colors(cx).glass_surface_hover;
        element
            .when(selected, |entry| entry.bg(selected_fill))
            .hover(move |style| style.bg(if selected { selected_fill.blend(hover) } else { hover }))
            .active(move |style| style.bg(selected_fill.blend(hover)))
    }
}

pub struct GlassSearchBar;

impl GlassSearchBar {
    pub fn wrap(element: Div, cx: &App) -> Div {
        let colors = design_system::active_colors(cx);
        element
            .rounded(px(RADIUS.control))
            .border(px(BORDER.width))
            .border_color(colors.glass_border)
            .bg(colors.glass_surface)
    }

    pub fn field(input: impl IntoElement, cx: &App) -> Div {
        let colors = design_system::active_colors(cx);
        h_flex()
            .gap_2()
            .items_center()
            .child(Icon::new(crate::assets::MaterialIcon::Search).size(px(20.)).text_color(colors.text_muted))
            .child(div().flex_1().child(input))
    }
}

pub struct GlassDialog;

impl GlassDialog {
    pub fn style(dialog: Dialog, cx: &App) -> Dialog {
        let colors = design_system::active_colors(cx);
        dialog
            .rounded(px(RADIUS.dialog))
            .border(px(BORDER.width))
            .border_color(colors.glass_border)
            .bg(colors.glass_surface)
            .shadow(vec![SHADOW_MODAL.to_box_shadow()])
    }
}

pub struct GlassPopup;

impl GlassPopup {
    pub fn style(
        mut style: PopupMenuStyle,
        background: Hsla,
        border: Hsla,
        highlight: Hsla,
    ) -> PopupMenuStyle {
        style.background = Some(background);
        style.radius = Some(px(RADIUS.control));
        style.shadow = Some(vec![
            BoxShadow::new(px(0.), px(0.), border).spread_radius(px(1.)).inset(),
            SHADOW_PANEL.to_box_shadow(),
        ]);
        style.highlight = Some(highlight);
        style
    }
}

pub struct GlassPlayerPanel;

impl GlassPlayerPanel {
    pub fn wrap(element: Div, cx: &App) -> Div {
        let colors = design_system::active_colors(cx);
        let ink = design_system::colors(true, false);
        element
            .bg(colors.player_surface)
            .border_color(colors.glass_border)
            .border_t(px(BORDER.width))
            .text_color(ink.text_primary)
    }
}
