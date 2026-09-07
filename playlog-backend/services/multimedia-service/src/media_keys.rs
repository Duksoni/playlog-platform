use crate::model::{
    FieldName,
    FieldName::{Cover, Screenshot, Trailer},
    GameMedia, MediaFile,
};
use std::{collections::HashSet, time::SystemTime};

pub fn game_prefix(game_id: i32) -> String {
    format!("games/{game_id}/")
}

pub fn staged_object_key(
    game_id: i32,
    field: FieldName,
    file_name: &str,
    screenshot_index: Option<usize>,
    attempt: &str,
) -> String {
    let ext = file_name.rsplit('.').next().unwrap_or("bin");

    match field {
        Cover => format!("games/{game_id}/cover_{attempt}.{ext}"),
        Trailer => format!("games/{game_id}/trailer_{attempt}.{ext}"),
        Screenshot => {
            let seq = screenshot_index.unwrap_or_else(|| {
                SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as usize
            });
            format!("games/{game_id}/screenshot_{attempt}_{seq}.{ext}")
        }
    }
}

pub fn unique_attempt_suffix() -> String {
    mongodb::bson::oid::ObjectId::new().to_hex()
}

pub fn uploaded_keys(
    cover: &Option<MediaFile>,
    screenshots: &Option<Vec<MediaFile>>,
    trailer: &Option<MediaFile>,
) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(cover) = cover {
        keys.push(cover.object_key.clone());
    }
    if let Some(screenshots) = screenshots {
        keys.extend(
            screenshots
                .iter()
                .map(|screenshot| screenshot.object_key.clone()),
        );
    }
    if let Some(trailer) = trailer {
        keys.push(trailer.object_key.clone());
    }
    keys
}

pub fn replaced_keys(
    existing: &GameMedia,
    new_cover: &Option<MediaFile>,
    new_screenshots: &Option<Vec<MediaFile>>,
    new_trailer: &Option<MediaFile>,
) -> Vec<String> {
    let mut keys = Vec::new();

    if let Some(new_cover) = new_cover
        && let Some(existing_cover) = &existing.cover
        && new_cover.object_key != existing_cover.object_key
    {
        keys.push(existing_cover.object_key.clone());
    }

    if let Some(new_screenshots) = new_screenshots {
        let retained: HashSet<&str> = new_screenshots
            .iter()
            .map(|screenshot| screenshot.object_key.as_str())
            .collect();
        keys.extend(
            existing
                .screenshots
                .iter()
                .filter(|screenshot| !retained.contains(screenshot.object_key.as_str()))
                .map(|screenshot| screenshot.object_key.clone()),
        );
    }

    if let Some(new_trailer) = new_trailer
        && let Some(existing_trailer) = &existing.trailer
        && new_trailer.object_key != existing_trailer.object_key
    {
        keys.push(existing_trailer.object_key.clone());
    }

    keys
}

pub fn filter_unknown_keys(known_keys: &[String], listed_keys: Vec<String>) -> Vec<String> {
    let known: HashSet<&str> = known_keys.iter().map(String::as_str).collect();
    listed_keys
        .into_iter()
        .filter(|key| !known.contains(key.as_str()))
        .collect()
}
