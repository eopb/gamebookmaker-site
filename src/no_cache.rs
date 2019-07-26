//! Data structure to send templates without cache.

use rocket::{http::Header, response::Responder};

#[derive(Responder)]
#[response(status = 200, content_type = "html")]
pub struct Template {
    inner: rocket_contrib::templates::Template,
    cache_control: Header<'static>,
    pragma: Header<'static>,
    expires: Header<'static>,
}

impl Template {
    pub fn with(template: rocket_contrib::templates::Template) -> Self {
        Self {
            inner: template,
            cache_control: Header::new("Cache-Control", "no-cache, no-store, must-revalidate"),
            pragma: Header::new("Pragma", "no-cache"),
            expires: Header::new("Expires", "0"),
        }
    }
}
