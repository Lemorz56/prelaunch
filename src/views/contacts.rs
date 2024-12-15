use loco_rs::prelude::*;

use crate::models::_entities::contacts;

/// Render a list view of contacts.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<contacts::Model>) -> Result<Response> {
    format::render().view(v, "contacts/list.html", data!({"items": items}))
}

/// Render a single contacts view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &contacts::Model) -> Result<Response> {
    format::render().view(v, "contacts/show.html", data!({"item": item}))
}

/// Render a contacts create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "contacts/create.html", data!({}))
}

/// Render a contacts edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &contacts::Model) -> Result<Response> {
    format::render().view(v, "contacts/edit.html", data!({"item": item}))
}
