use std::str::FromStr;

use super::{UseLayout, UseVerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    state::{PreState, State},
};
use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Anchor {
    Start,
    Middle,
    End,
}

#[derive(Clone, Debug)]
pub struct RotatedLabel {
    pub text: RwSignal<String>,
    pub anchor: RwSignal<Anchor>,
}

impl RotatedLabel {
    pub fn new(anchor: impl Into<RwSignal<Anchor>>, text: impl Into<RwSignal<String>>) -> Self {
        Self {
            text: text.into(),
            anchor: anchor.into(),
        }
    }

    pub fn start(text: impl Into<String>) -> Self {
        Self::new(Anchor::Start, create_rw_signal(text.into()))
    }
    pub fn middle(text: impl Into<String>) -> Self {
        Self::new(Anchor::Middle, create_rw_signal(text.into()))
    }
    pub fn end(text: impl Into<String>) -> Self {
        Self::new(Anchor::End, create_rw_signal(text.into()))
    }

    fn size<X, Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let text = self.text;
        let font_height = state.font_height;
        let padding = state.padding;
        Signal::derive(move || {
            if text.with(|t| t.is_empty()) {
                0.0
            } else {
                font_height.get() + padding.get().height()
            }
        })
    }

    pub(super) fn fixed_height<X, Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        self.size(state)
    }

    pub(super) fn to_horizontal_use(&self) -> UseLayout {
        UseLayout::RotatedLabel(self.clone())
    }

    pub(super) fn to_vertical_use<X, Y>(&self, state: &PreState<X, Y>) -> UseVerticalLayout {
        // Note: width is height because it's rotated
        UseVerticalLayout {
            width: self.size(state),
            layout: UseLayout::RotatedLabel(self.clone()),
        }
    }
}

impl Anchor {
    fn to_svg_attr(self) -> String {
        self.to_string()
    }

    fn map_points(&self, left: f64, middle: f64, right: f64) -> f64 {
        match self {
            Anchor::Start => left,
            Anchor::Middle => middle,
            Anchor::End => right,
        }
    }

    pub fn css_justify_content(&self) -> &'static str {
        match self {
            Anchor::Start => "flex-start",
            Anchor::Middle => "center",
            Anchor::End => "flex-end",
        }
    }
}

impl FromStr for Anchor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "start" => Ok(Anchor::Start),
            "middle" => Ok(Anchor::Middle),
            "end" => Ok(Anchor::End),
            _ => Err(format!("unknown anchor: `{}`", s)),
        }
    }
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Anchor::Start => write!(f, "start"),
            Anchor::Middle => write!(f, "middle"),
            Anchor::End => write!(f, "end"),
        }
    }
}

#[component]
pub(super) fn RotatedLabel<X: 'static, Y: 'static>(
    label: RotatedLabel,
    edge: Edge,
    bounds: Memo<Bounds>,
    state: State<X, Y>,
) -> impl IntoView {
    let RotatedLabel { text, anchor } = label;
    let debug = state.pre.debug;
    let font_height = state.pre.font_height;
    let padding = state.pre.padding;

    let content = Signal::derive(move || padding.get().apply(bounds.get()));
    let position = create_memo(move |_| {
        let c = content.get();
        let (top, right, bottom, left) = (c.top_y(), c.right_x(), c.bottom_y(), c.left_x());
        let (centre_x, centre_y) = (c.centre_x(), c.centre_y());

        let anchor = anchor.get();
        match edge {
            Edge::Top | Edge::Bottom => (0, anchor.map_points(left, centre_x, right), centre_y),
            Edge::Left => (270, centre_x, anchor.map_points(bottom, centre_y, top)),
            // Right rotates the opposite way to Left inverting the anchor points
            Edge::Right => (90, centre_x, anchor.map_points(top, centre_y, bottom)),
        }
    });

    view! {
        <g class="_chartistry_rotated_label">
            <DebugRect label="RotatedLabel" debug=debug bounds=vec![bounds.into(), content] />
            <text
                x=move || position.with(|(_, x, _)| x.to_string())
                y=move || position.with(|(_, _, y)| y.to_string())
                transform=move || position.with(|(rotate, x, y)| format!("rotate({rotate}, {x}, {y})"))
                dominant-baseline="middle"
                text-anchor=move || anchor.get().to_svg_attr()
                font-family="monospace"
                font-size=move || format!("{}px", font_height.get())>
                {text}
            </text>
        </g>
    }
}
