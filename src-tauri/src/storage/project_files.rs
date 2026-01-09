use serde_json::json;

use crate::models::project;

pub fn project_json(proj: &project::Project) -> serde_json::Value {
    let obj =json!({
        "name": proj.name,
        "modId": proj.mod_id,
        "description": proj.description,
        "author": proj.author,
        "version": proj.target_version,
        "generators": json!(proj.generators)
    });
    obj
}