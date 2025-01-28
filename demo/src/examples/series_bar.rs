use super::MyDataNegative;
use leptos::prelude::*;
use leptos_chartistry::*;

#[component]
pub fn Example(debug: Signal<bool>, data: Signal<Vec<MyDataNegative>>) -> impl IntoView {
    let series = Series::new(|data: &MyDataNegative| data.x)
        .bar(|data: &MyDataNegative| data.y);

    view! {
        <Chart
            aspect_ratio=AspectRatio::from_outer_height(300.0, 1.2)
            debug=debug
            series=series
            data=data

            left=TickLabels::aligned_floats()
            inner=[
                AxisMarker::left_edge().into_inner(),
                AxisMarker::bottom_edge().into_inner(),
                YGridLine::default().into_inner(),
            ]
        />
    }
}
