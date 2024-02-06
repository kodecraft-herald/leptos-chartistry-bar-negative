use super::{rotated_label::Anchor, UseLayout, UseVerticalLayout};
use crate::{
    bounds::Bounds,
    debug::DebugRect,
    edge::Edge,
    series::{Snippet, UseLine},
    state::{PreState, State},
    Padding,
};
use leptos::*;

#[derive(Clone, Debug)]
pub struct Legend {
    pub anchor: RwSignal<Anchor>,
}

impl Legend {
    pub fn new(anchor: impl Into<RwSignal<Anchor>>) -> Self {
        Self {
            anchor: anchor.into(),
        }
    }

    pub fn start() -> Legend {
        Self::new(Anchor::Start)
    }
    pub fn middle() -> Legend {
        Self::new(Anchor::Middle)
    }
    pub fn end() -> Legend {
        Self::new(Anchor::End)
    }

    pub(crate) fn width<X, Y>(state: &PreState<X, Y>) -> Signal<f64> {
        let font_height = state.font_height;
        let font_width = state.font_width;
        let padding = state.padding;
        let series = state.data.series;
        let snippet_bounds = UseLine::snippet_width(font_height, font_width);
        Signal::derive(move || {
            let font_width = font_width.get();
            let max_chars = series
                .get()
                .into_iter()
                .map(|line| line.name.get().len() as f64 * font_width)
                .reduce(f64::max)
                .unwrap_or_default();
            snippet_bounds.get() + max_chars + padding.get().width()
        })
    }

    pub(crate) fn fixed_height<X, Y>(&self, state: &PreState<X, Y>) -> Signal<f64> {
        let font_height = state.font_height;
        let padding = state.padding;
        Signal::derive(move || font_height.get() + padding.get().height())
    }

    pub(super) fn to_horizontal_use(&self) -> UseLayout {
        UseLayout::Legend(self.clone())
    }

    pub(super) fn to_vertical_use<X, Y>(&self, state: &PreState<X, Y>) -> UseVerticalLayout {
        UseVerticalLayout {
            width: Self::width(state),
            layout: UseLayout::Legend(self.clone()),
        }
    }
}

#[component]
pub(crate) fn Legend<X: Clone + 'static, Y: Clone + 'static>(
    legend: Legend,
    #[prop(into)] edge: MaybeSignal<Edge>,
    bounds: Memo<Bounds>,
    state: State<X, Y>,
) -> impl IntoView {
    let anchor = legend.anchor;
    let debug = state.pre.debug;
    let font_height = state.pre.font_height;
    let padding = state.pre.padding;
    let series = state.pre.data.series;

    // Don't apply padding on the edges of our axis i.e., maximise the space we extend over
    let padding = create_memo(move |_| {
        let padding = padding.get();
        if edge.get().is_horizontal() {
            Padding::sides(padding.top, 0.0, padding.bottom, 0.0)
        } else {
            Padding::sides(0.0, padding.right, 0.0, padding.left)
        }
    });
    let inner = Signal::derive(move || padding.get().apply(bounds.get()));

    let html = move || {
        let edge = edge.get();
        let body = if edge.is_horizontal() {
            view!(<HorizontalBody series=series state=state.clone() />)
        } else {
            view!(<VerticalBody series=series state=state.clone() />)
        };
        view! {
            <div
                style="display: flex; height: 100%; overflow: auto;"
                style:flex-direction={if edge.is_horizontal() { "row" } else { "column" }}
                style:justify-content=move || anchor.get().css_justify_content()>
                <table
                    style="border-collapse: collapse; border-spacing: 0; margin: 0;"
                    style:font-size=move || format!("{}px", font_height.get())>
                    <tbody>
                        {body}
                    </tbody>
                </table>
            </div>
        }
    };

    view! {
        <g class="_chartistry_legend">
            <DebugRect label="Legend" debug=debug bounds=vec![bounds.into(), inner] />
            <foreignObject
                x=move || bounds.get().left_x()
                y=move || bounds.get().top_y()
                width=move || bounds.get().width()
                height=move || bounds.get().height()
                style="overflow: visible;">
                {html}
            </foreignObject>
        </g>
    }
}

#[component]
fn VerticalBody<X: Clone + 'static, Y: Clone + 'static>(
    series: Memo<Vec<UseLine>>,
    state: State<X, Y>,
) -> impl IntoView {
    let padding = state.pre.padding;
    let state = state.clone();
    view! {
        <For
            each=move || series.get()
            key=|series| series.id
            let:series>
            <tr>
                <td style:padding=move || padding.get().to_css_horizontal_style()>
                    <Snippet series=series state=state.clone() />
                </td>
            </tr>
        </For>
    }
}

#[component]
fn HorizontalBody<X: Clone + 'static, Y: Clone + 'static>(
    series: Memo<Vec<UseLine>>,
    state: State<X, Y>,
) -> impl IntoView {
    let padding = state.pre.padding;
    let padding = move |i| -> Option<String> {
        if i != 0 {
            Some(format!("{}px", padding.get().left))
        } else {
            None
        }
    };
    let state = state.clone();
    view! {
        <tr>
            <For
                each=move || series.get().into_iter().enumerate()
                key=|(_, series)| series.id
                let:series>
                <td style:padding-left=move || padding(series.0)>
                    <Snippet series=series.1 state=state.clone() />
                </td>
            </For>
        </tr>
    }
}
