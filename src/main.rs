#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use freya::prelude::*;

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    let mut times = use_state(cx, || 1);

    let exclamations = "!".repeat(*times.get());

    render!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(0, 109, 119)",
            direction: "vertical",
            display: "center",
            onclick: move |_| times += 1,
            label {
                width: "100%",
                font_size: "50",
                align: "center",
                color: "white",
                "Hello, World{exclamations}"
            }
        }
    )
}
