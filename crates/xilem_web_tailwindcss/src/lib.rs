//! TailwindCSS class helpers for `xilem_web`.
//!
//! The `tw!` macro splits whitespace into class tokens and returns a
//! `TailwindClasses` list.
//!
//! # Example
//!
//! ```rust,ignore
//! use xilem_web::elements::html::div;
//! use xilem_web::interfaces::Element as _;
//! use xilem_web_tailwindcss::tw;
//!
//! fn view(active: bool) -> impl xilem_web::interfaces::Element<()> {
//!     div("Hello").class(tw!(
//!         "px-4 py-2 text-sm",
//!         if active => "bg-blue-600 text-white",
//!         if !active => "bg-gray-200 text-gray-900",
//!     ))
//! }
//! ```

use std::borrow::Cow;

/// A class token type compatible with `xilem_web::modifiers::ClassIter`.
pub type CowStr = Cow<'static, str>;

/// A whitespace-split Tailwind class list.
pub type TailwindClasses = Vec<CowStr>;

/// Inputs that can be appended to a Tailwind class list.
pub trait TwInput {
    /// Append this input to an existing class list.
    fn append_to(self, classes: &mut TailwindClasses);
}

/// Build a Tailwind class list from a single input.
pub fn tw(input: impl TwInput) -> TailwindClasses {
    let mut classes = Vec::new();
    input.append_to(&mut classes);
    classes
}

#[doc(hidden)]
pub fn __tw_push_literal(classes: &mut TailwindClasses, input: &'static str) {
    classes.extend(input.split_whitespace().map(Cow::Borrowed));
}

#[doc(hidden)]
pub fn __tw_push_str(classes: &mut TailwindClasses, input: &str) {
    classes.extend(
        input
            .split_whitespace()
            .map(|token| Cow::Owned(token.to_string())),
    );
}

impl TwInput for CowStr {
    fn append_to(self, classes: &mut TailwindClasses) {
        classes.push(self);
    }
}

impl<'a> TwInput for &'a TailwindClasses {
    fn append_to(self, classes: &mut TailwindClasses) {
        classes.extend(self.iter().cloned());
    }
}

impl<'a> TwInput for &'a str {
    fn append_to(self, classes: &mut TailwindClasses) {
        __tw_push_str(classes, self);
    }
}

impl TwInput for String {
    fn append_to(self, classes: &mut TailwindClasses) {
        __tw_push_str(classes, &self);
    }
}

impl<T: TwInput> TwInput for Option<T> {
    fn append_to(self, classes: &mut TailwindClasses) {
        if let Some(value) = self {
            value.append_to(classes);
        }
    }
}

impl<T: TwInput> TwInput for Vec<T> {
    fn append_to(self, classes: &mut TailwindClasses) {
        for value in self {
            value.append_to(classes);
        }
    }
}

impl<T: TwInput, const N: usize> TwInput for [T; N] {
    fn append_to(self, classes: &mut TailwindClasses) {
        for value in self {
            value.append_to(classes);
        }
    }
}

#[macro_export]
macro_rules! tw {
    () => {
        ::std::vec::Vec::new()
    };
    ($($rest:tt)+) => {{
        let mut classes = ::std::vec::Vec::new();
        $crate::__tw_internal!(@append classes; $($rest)+);
        classes
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __tw_internal {
    (@append $classes:ident; ) => {};
    (@append $classes:ident; if $cond:expr => $value:literal , $($rest:tt)*) => {{
        if $cond {
            $crate::__tw_push_literal(&mut $classes, $value);
        }
        $crate::__tw_internal!(@append $classes; $($rest)*);
    }};
    (@append $classes:ident; if $cond:expr => $value:literal) => {{
        if $cond {
            $crate::__tw_push_literal(&mut $classes, $value);
        }
    }};
    (@append $classes:ident; if $cond:expr => $value:expr , $($rest:tt)*) => {{
        if $cond {
            $crate::TwInput::append_to($value, &mut $classes);
        }
        $crate::__tw_internal!(@append $classes; $($rest)*);
    }};
    (@append $classes:ident; if $cond:expr => $value:expr) => {{
        if $cond {
            $crate::TwInput::append_to($value, &mut $classes);
        }
    }};
    (@append $classes:ident; $value:literal , $($rest:tt)*) => {{
        $crate::__tw_push_literal(&mut $classes, $value);
        $crate::__tw_internal!(@append $classes; $($rest)*);
    }};
    (@append $classes:ident; $value:literal) => {{
        $crate::__tw_push_literal(&mut $classes, $value);
    }};
    (@append $classes:ident; $value:expr , $($rest:tt)*) => {{
        $crate::TwInput::append_to($value, &mut $classes);
        $crate::__tw_internal!(@append $classes; $($rest)*);
    }};
    (@append $classes:ident; $value:expr) => {{
        $crate::TwInput::append_to($value, &mut $classes);
    }};
}

#[cfg(test)]
mod tests {
    use super::{TailwindClasses, tw};
    use std::borrow::Cow;

    #[test]
    fn tw_splits_literals() {
        let classes = tw!("p-4 text-sm", "bg-blue-500");
        let expected: TailwindClasses = vec![
            Cow::Borrowed("p-4"),
            Cow::Borrowed("text-sm"),
            Cow::Borrowed("bg-blue-500"),
        ];
        assert_eq!(classes, expected);
    }

    #[test]
    fn tw_conditional_literals() {
        let active = true;
        let classes = tw!("base", if active => "active", if !active => "inactive");
        let expected: TailwindClasses = vec![Cow::Borrowed("base"), Cow::Borrowed("active")];
        assert_eq!(classes, expected);
    }

    #[test]
    fn tw_expression_splits() {
        let input = String::from("p-4 text-sm");
        let classes = tw!(input);
        let expected: TailwindClasses = vec![
            Cow::Owned("p-4".to_string()),
            Cow::Owned("text-sm".to_string()),
        ];
        assert_eq!(classes, expected);
    }

    #[test]
    fn tw_with_array_and_option() {
        let optional = Some("text-sm");
        let classes = tw!(["p-4", "bg-blue-500"], optional);
        let expected: TailwindClasses = vec![
            Cow::Owned("p-4".to_string()),
            Cow::Owned("bg-blue-500".to_string()),
            Cow::Owned("text-sm".to_string()),
        ];
        assert_eq!(classes, expected);
    }

    #[test]
    fn tw_function_splits_str() {
        let classes = tw("p-4 text-sm");
        let expected: TailwindClasses = vec![
            Cow::Owned("p-4".to_string()),
            Cow::Owned("text-sm".to_string()),
        ];
        assert_eq!(classes, expected);
    }
}
