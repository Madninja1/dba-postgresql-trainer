use crate::{ContentBundle, ContentError, load_bundle};

include!(concat!(env!("OUT_DIR"), "/builtin_registry.rs"));

pub fn load_builtin_bundles() -> Result<Vec<ContentBundle>, ContentError> {
    BUILTIN_DOCUMENTS
        .iter()
        .map(|(topic_json, questions_json)| load_bundle(topic_json, questions_json))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_content_is_valid() {
        let bundles = load_builtin_bundles().expect("builtin content should be valid");

        assert!(!bundles.is_empty());

        let tools_topic = bundles
            .iter()
            .find(|bundle| bundle.topic.slug == "dba1-tools-install")
            .expect("tools and installation topic should exist");

        assert_eq!(tools_topic.questions.questions.len(), 24);
    }
}
