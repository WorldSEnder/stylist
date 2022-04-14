#[cfg(feature = "parser")]
use std::borrow::Cow;

use crate::ast::Sheet;
use crate::manager::StyleManager;
#[cfg(feature = "yew")]
use crate::Style;

#[derive(Debug, Clone, PartialEq)]
enum SheetSource {
    Sheet(Sheet),
}

/// A struct that can be used as a source to create a [`Style`](crate::Style) or
/// [`GlobalStyle`](crate::GlobalStyle).
///
/// This struct is usually created by [`css!`](crate::css) macro.
///
/// You can also get a StyleSource instance from a string or a [`Sheet`] by calling `.into()`.
///
/// ```rust
/// use yew::prelude::*;
/// use stylist::{css, StyleSource, yew::Global};
///
/// let s: StyleSource = css!("color: red;");
///
/// let rendered = html! {<div class={s.clone()} />};
/// let global_rendered = html! {<Global css={s} />};
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSource {
    #[cfg(feature = "parser")]
    inner: SheetSource,

    manager: Option<StyleManager>,
    #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
    pub(crate) location: String,
}

impl StyleSource {
    #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
    #[track_caller]
    pub(crate) fn get_caller_location() -> String {
        let caller_loc = std::panic::Location::caller();
        // Who cares if this a valid css class name, it's debugging info
        format!(
            "{}:{}:{}",
            caller_loc.file(),
            caller_loc.line(),
            caller_loc.column()
        )
    }

    pub(crate) fn into_sheet(self) -> Sheet {
        match self.inner {
            SheetSource::Sheet(m) => m,
        }
    }

    #[cfg(feature = "yew")]
    pub(crate) fn into_style(mut self) -> Style {
        use stylist_core::ResultDisplay;
        let manager = self.manager.take().unwrap_or_default();
        Style::new_with_manager(self, manager).expect_display("Failed to create style")
    }

    #[doc(hidden)]
    pub fn with_manager(mut self, manager: StyleManager) -> Self {
        self.manager = Some(manager);

        self
    }
}

impl From<Sheet> for StyleSource {
    #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
    fn from(other: Sheet) -> StyleSource {
        StyleSource {
            inner: SheetSource::Sheet(other),
            manager: None,
            #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
            location: Self::get_caller_location(),
        }
    }
}

#[cfg_attr(documenting, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
mod feat_parser {
    use super::*;

    impl TryFrom<String> for StyleSource {
        type Error = crate::Error;
        #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
        fn try_from(other: String) -> crate::Result<StyleSource> {
            let sheet = SheetSource::Sheet(other.parse()?);
            Ok(StyleSource {
                inner: sheet,
                manager: None,
                #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
                location: Self::get_caller_location(),
            })
        }
    }

    impl<'a> TryFrom<&'a str> for StyleSource {
        type Error = crate::Error;
        #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
        fn try_from(other: &'a str) -> crate::Result<StyleSource> {
            let sheet = SheetSource::Sheet(other.parse()?);
            Ok(StyleSource {
                inner: sheet,
                manager: None,
                #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
                location: Self::get_caller_location(),
            })
        }
    }

    impl<'a> TryFrom<Cow<'a, str>> for StyleSource {
        type Error = crate::Error;
        #[cfg_attr(all(debug_assertions, feature = "debug_style_locations"), track_caller)]
        fn try_from(other: Cow<'a, str>) -> crate::Result<StyleSource> {
            let sheet = SheetSource::Sheet(other.parse()?);
            Ok(StyleSource {
                inner: sheet,
                manager: None,
                #[cfg(all(debug_assertions, feature = "debug_style_locations"))]
                location: Self::get_caller_location(),
            })
        }
    }
}
