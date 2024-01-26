use super::UseInner;
use crate::{
    colours::Colour, debug::DebugRect, projection::Projection, state::State, ticks::GeneratedTicks,
    Tick, TickLabels,
};
use leptos::*;
use std::{borrow::Borrow, rc::Rc};

macro_rules! impl_grid_line {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name<Tick: 'static> {
            width: MaybeSignal<f64>,
            colour: MaybeSignal<Option<Colour>>,
            ticks: TickLabels<Tick>,
        }

        impl<Tick> $name<Tick> {
            pub fn new(ticks: impl Borrow<TickLabels<Tick>>) -> Self {
                Self {
                    width: 1.0.into(),
                    colour: MaybeSignal::default(),
                    ticks: ticks.borrow().clone(),
                }
            }

            pub fn set_width(mut self, width: impl Into<MaybeSignal<f64>>) -> Self {
                self.width = width.into();
                self
            }

            pub fn set_colour(mut self, colour: impl Into<MaybeSignal<Option<Colour>>>) -> Self {
                self.colour = colour.into();
                self
            }
        }
    };
}

impl_grid_line!(HorizontalGridLine);
impl_grid_line!(VerticalGridLine);

macro_rules! impl_use_grid_line {
    ($name:ident) => {
        struct $name<Tick: 'static> {
            width: MaybeSignal<f64>,
            colour: MaybeSignal<Option<Colour>>,
            ticks: Signal<GeneratedTicks<Tick>>,
        }

        impl<Tick> Clone for $name<Tick> {
            fn clone(&self) -> Self {
                Self {
                    width: self.width.clone(),
                    colour: self.colour.clone(),
                    ticks: self.ticks,
                }
            }
        }
    };
}

impl_use_grid_line!(UseHorizontalGridLine);
impl_use_grid_line!(UseVerticalGridLine);

impl<X: Tick> HorizontalGridLine<X> {
    pub(crate) fn use_horizontal<Y>(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        let inner = state.layout.inner;
        let avail_width = Signal::derive(move || with!(|inner| inner.width()));
        Rc::new(UseHorizontalGridLine {
            width: self.width,
            colour: self.colour,
            ticks: self.ticks.generate_x(&state.pre, avail_width),
        })
    }
}

impl<Y: Tick> VerticalGridLine<Y> {
    pub(crate) fn use_vertical<X>(self, state: &State<X, Y>) -> Rc<dyn UseInner<X, Y>> {
        let inner = state.layout.inner;
        let avail_height = Signal::derive(move || with!(|inner| inner.height()));
        Rc::new(UseVerticalGridLine {
            width: self.width,
            colour: self.colour,
            ticks: self.ticks.generate_y(&state.pre, avail_height),
        })
    }
}

impl<X, Y> UseInner<X, Y> for UseHorizontalGridLine<X> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <ViewHorizontalGridLine line=(*self).clone() state=state /> )
    }
}

impl<X, Y> UseInner<X, Y> for UseVerticalGridLine<Y> {
    fn render(self: Rc<Self>, state: State<X, Y>) -> View {
        view!( <ViewVerticalGridLine line=(*self).clone() state=state /> )
    }
}

#[component]
fn ViewHorizontalGridLine<X: 'static, Y: 'static>(
    line: UseHorizontalGridLine<X>,
    state: State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let inner = state.layout.inner;
    let proj = state.projection;

    let colour = Colour::signal_option(line.colour, super::DEFAULT_COLOUR_GRID_LINE);
    view! {
        <g class="_chartistry_grid_line_x">
            <DebugRect label="grid_line_x" debug=debug />
            <For
                each=move || for_ticks(line.ticks, proj, true)
                key=|(_, label)| label.to_owned()
                let:tick
            >
                <DebugRect label=format!("grid_line_x/{}", tick.1) debug=debug />
                <line
                    x1=tick.0
                    y1=move || inner.get().top_y()
                    x2=tick.0
                    y2=move || inner.get().bottom_y()
                    stroke=move || colour.get().to_string()
                    stroke-width=line.width
                />
            </For>
        </g>
    }
}

#[component]
fn ViewVerticalGridLine<X: 'static, Y: 'static>(
    line: UseVerticalGridLine<Y>,
    state: State<X, Y>,
) -> impl IntoView {
    let debug = state.pre.debug;
    let inner = state.layout.inner;
    let proj = state.projection;

    let colour = Colour::signal_option(line.colour, super::DEFAULT_COLOUR_GRID_LINE);
    view! {
        <g class="_chartistry_grid_line_y">
            <DebugRect label="grid_line_y" debug=debug />
            <For
                each=move || for_ticks(line.ticks, proj, false)
                key=|(_, label)| label.to_owned()
                let:tick
            >
                <DebugRect label=format!("grid_line_y/{}", tick.1) debug=debug />
                <line
                    x1=move || inner.get().left_x()
                    y1=tick.0
                    x2=move || inner.get().right_x()
                    y2=tick.0
                    stroke=move || colour.get().to_string()
                    stroke-width=line.width
                />
            </For>
        </g>
    }
}

fn for_ticks<Tick>(
    ticks: Signal<GeneratedTicks<Tick>>,
    proj: Signal<Projection>,
    is_x: bool,
) -> Vec<(f64, String)> {
    ticks.with(move |ticks| {
        let proj = proj.get();
        ticks
            .ticks
            .iter()
            .map(|tick| {
                let label = ticks.state.format(tick);
                let tick = ticks.state.position(tick);
                let tick = if is_x {
                    proj.position_to_svg(tick, 0.0).0
                } else {
                    proj.position_to_svg(0.0, tick).1
                };
                (tick, label)
            })
            .collect::<Vec<_>>()
    })
}
