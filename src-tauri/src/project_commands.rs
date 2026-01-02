use crate::models;

//TODO: read from a "recent" list
#[tauri::command]
pub async fn get_recent_projects() -> Vec<models::ProjectData> {
    vec![
        models::ProjectData {
            name: "InspectAnims".into(),
            path: "C:/Users/alex/mods/CSINSPECTMOD".into(),
            target_version: models::Version {
                major: 26,
                minor: 1,
                patch: 0,
            },
        },
        models::ProjectData {
            name: "Optium".into(),
            path: "C:/Users/alex/mods/optimization/optium".into(),
            target_version: models::Version {
                major: 26,
                minor: 1,
                patch: 0,
            },
        },
        models::ProjectData {
            name: "WarpathMod".into(),
            path: "C:/Users/alex/mods/servers/majorFFA".into(),
            target_version: models::Version {
                major: 26,
                minor: 1,
                patch: 1,
            },
        },
    ]
}
#[tauri::command]
pub async fn create_project(name: String, path: String, selected_version: String) -> () {
    println!(
        "Creating project {} at {} for Fabric {}",
        name, path, selected_version
    );
}
