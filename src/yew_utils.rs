use crate::*;
use yew::*;

pub fn svg_line(line: &Line, class: &str) -> Html {
    html! {
        <line x1={line.x_start} y1={line.y_start} x2={line.x_end} y2={line.y_end} class={class} />
    }
}

pub fn svg_lines<'a>(lines: impl Iterator<Item = (&'a Line, &'a str)>) -> Vec<Html> {
    lines.map(|l| svg_line(l.0, l.1)).collect()
}

pub fn svg_label(label: &Label, class: &str) -> Html {
    html! {
        <text x={label.x} y={label.y} class={class}>{&label.text}</text>
    }
}

pub fn svg_labels<'a>(labels: impl Iterator<Item = (&'a Label, &'a str)>) -> Vec<Html> {
    labels.map(|l| svg_label(l.0, l.1)).collect()
}
