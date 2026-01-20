// Copyright 2025 the Xilem Tailwind CSS Authors
// SPDX-License-Identifier: Apache-2.0

//! Tailwind CSS + xilem_web example
//!
//! This example demonstrates how to use Tailwind CSS with xilem_web
//! using the `tw!` macro from `xilem_web_tailwindcss`.

use xilem_web::core::Edit;
use xilem_web::elements::html as el;
use xilem_web::interfaces::{Element as _, HtmlButtonElement, HtmlInputElement};
use xilem_web::{document_body, App, DomFragment};
use xilem_web_tailwindcss::tw;
use web_sys::wasm_bindgen::JsCast;

#[derive(Default)]
struct AppState {
    count: i32,
    name: String,
    dark_mode: bool,
}

/// A styled button component
fn button<F: Fn(&mut AppState, web_sys::PointerEvent) + 'static>(
    label: &'static str,
    click_fn: F,
) -> impl HtmlButtonElement<Edit<AppState>> {
    el::button::<Edit<AppState>, _, _>(label)
        .class(tw!(
            "inline-flex items-center justify-center rounded-lg px-4 py-2",
            "text-sm font-semibold transition-colors",
            "bg-indigo-600 text-white",
            "hover:bg-indigo-500 active:bg-indigo-700",
            "focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
        ))
        .on_click::<F, _>(click_fn)
}

/// A styled text input component
fn text_input(
    value: &str,
    placeholder: &'static str,
) -> impl HtmlInputElement<Edit<AppState>> + use<> {
    el::input(())
        .attr("type", "text")
        .attr("value", value.to_string())
        .attr("placeholder", placeholder)
        .class(tw!(
            "w-full rounded-lg border border-slate-300 bg-white px-4 py-2",
            "text-slate-900 placeholder:text-slate-400",
            "focus:border-indigo-500 focus:outline-none focus:ring-2 focus:ring-indigo-500/20"
        ))
        .on_input(|state: &mut AppState, event| {
            let target: web_sys::HtmlInputElement = event.target().unwrap().unchecked_into();
            state.name = target.value();
        })
}

fn app_logic(state: &mut AppState) -> impl DomFragment<Edit<AppState>> + use<> {
    let greeting = if state.name.is_empty() {
        "Hello, World!".to_string()
    } else {
        format!("Hello, {}!", state.name)
    };

    let dark = state.dark_mode;

    let container_classes = tw!(
        "min-h-screen transition-colors",
        if dark => "bg-slate-900 text-slate-100",
        if !dark => "bg-slate-50 text-slate-900"
    );

    let card_classes = tw!(
        "w-full max-w-md rounded-2xl border p-8 shadow-xl",
        if dark => "border-slate-700 bg-slate-800",
        if !dark => "border-slate-200 bg-white"
    );

    el::div(
        el::div(
            el::div((
                // Header
                el::div((
                    el::h1("Xilem Web + Tailwind")
                        .class(tw!("text-2xl font-bold tracking-tight")),
                    el::p("A reactive UI example with Tailwind CSS styling")
                        .class(tw!("mt-1 text-sm text-slate-500")),
                )),
                // Greeting section
                el::div((
                    el::p(greeting).class(tw!("text-xl font-medium")),
                    text_input(&state.name, "Enter your name..."),
                ))
                .class(tw!("mt-6 space-y-3")),
                // Counter section
                el::div((
                    el::p(format!("Count: {}", state.count))
                        .class(tw!("text-4xl font-bold tabular-nums")),
                    el::div((
                        button("-1", |state, _| state.count -= 1),
                        button("+1", |state, _| state.count += 1),
                        button("Reset", |state, _| state.count = 0),
                    ))
                    .class(tw!("flex gap-2")),
                ))
                .class(tw!("mt-6 space-y-3")),
                // Dark mode toggle
                el::div(
                    el::button::<Edit<AppState>, _, _>(if dark { "Light Mode" } else { "Dark Mode" })
                        .class(tw!(
                            "rounded-lg border border-slate-300 px-4 py-2",
                            "text-sm font-medium transition-colors hover:bg-slate-100"
                        ))
                        .on_click(|state: &mut AppState, _| {
                            state.dark_mode = !state.dark_mode;
                        }),
                )
                .class(tw!("mt-6")),
                // Footer
                el::div(
                    el::p("Built with xilem_web and Tailwind CSS v4")
                        .class(tw!("text-xs text-slate-400")),
                )
                .class(tw!("mt-8 pt-6 border-t border-slate-200")),
            ))
            .class(card_classes),
        )
        .class(tw!("flex items-center justify-center min-h-screen p-4")),
    )
    .class(container_classes)
}

fn main() {
    console_error_panic_hook::set_once();
    App::new(document_body(), AppState::default(), app_logic).run();
}
