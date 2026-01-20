# xilem_web_tailwindcss

TailwindCSS class helpers for `xilem_web`.

`xilem_web` expects class names as individual tokens. Tailwind utility strings are
usually written with whitespace, which makes them awkward to pass directly. This
crate provides a `tw!` macro (and `tw` function) that split whitespace into class
tokens and returns `Vec<Cow<'static, str>>`.

## Usage

```rust
use xilem_web::elements::html::div;
use xilem_web::interfaces::Element as _;
use xilem_web_tailwindcss::tw;

fn view(active: bool) -> impl xilem_web::interfaces::Element<()> {
    div("Hello").class(tw!(
        "px-4 py-2 text-sm",
        if active => "bg-blue-600 text-white",
        if !active => "bg-gray-200 text-gray-900",
    ))
}
```
