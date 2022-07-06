use perseus::{ErrorPages, Html};
use sycamore::view;

// This site will be exported statically, so we only have control over 404 pages
// for broken links in the site itself
pub fn get_error_pages<G: Html>() -> ErrorPages<G> {
    let mut error_pages = ErrorPages::new(|cx, url, status, err, _| {
        view! { cx,
            p { (format!("An error with HTTP code {} occurred at '{}': '{}'.", status, url, err)) }
        }
    });
    error_pages.add_page(404, |cx, _, _, _, _| {
        view! { cx,
            p { "Page not found." }
        }
    });

    error_pages
}
