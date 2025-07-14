use loco_rs::prelude::*;

use crate::models::_entities::beans;

/// Render a list view of `beans`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<beans::Model>) -> Result<Response> {
    format::render().view(v, "beans/list.html", data!({"items": items}))
}

/// Render a single `beans` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &beans::Model) -> Result<Response> {
    format::render().view(v, "beans/show.html", data!({"item": item}))
}

/// Render a `beans` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "beans/create.html", data!({}))
}

/// Render a `beans` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &beans::Model) -> Result<Response> {
    format::render().view(v, "beans/edit.html", data!({"item": item}))
}
