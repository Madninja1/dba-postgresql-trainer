use crate::{ContentBundle, ContentError, load_bundle};

const DBA1_TOOLS_TOPIC: &str =
    include_str!("../../../course-content/dba-1/01-tools-install/topic.json");

const DBA1_TOOLS_QUESTIONS: &str =
    include_str!("../../../course-content/dba-1/01-tools-install/questions.json");

pub fn load_builtin_bundles() -> Result<Vec<ContentBundle>, ContentError> {
    Ok(vec![load_bundle(DBA1_TOOLS_TOPIC, DBA1_TOOLS_QUESTIONS)?])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_content_is_valid() {
        let bundles = load_builtin_bundles().expect("builtin content should be valid");

        assert_eq!(bundles.len(), 1);

        assert_eq!(bundles[0].topic.course, "dba-1");

        assert_eq!(bundles[0].topic.slug, "dba1-tools-install");

        assert_eq!(bundles[0].questions.questions.len(), 12);
    }
}
