use loco_rs::prelude::*;

use crate::models::_entities::{espressos, beans};

/// Render a list view of `espressos`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<espressos::Model>) -> Result<Response> {
    format::render().view(v, "espresso/list.html", data!({"items": items}))
}

/// Render a single `espresso` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &espressos::Model) -> Result<Response> {
    format::render().view(v, "espresso/show.html", data!({"item": item}))
}

/// Render a `espresso` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer, beans: &Vec<beans::Model>) -> Result<Response> {
    format::render().view(v, "espresso/create.html", data!({"beans": beans}))
}

/// Render a `espresso` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &espressos::Model) -> Result<Response> {
    format::render().view(v, "espresso/edit.html", data!({"item": item}))
}
