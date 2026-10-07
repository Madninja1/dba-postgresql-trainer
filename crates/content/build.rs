use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct BundlePaths {
    topic_key: String,
    locale: String,
    topic_path: PathBuf,
    questions_path: PathBuf,
}

fn main() -> io::Result<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));

    let content_root = manifest_dir.join("../../course-content");

    let mut bundles = Vec::new();

    collect_bundles(&content_root, &content_root, &mut bundles)?;

    bundles.sort_by(|left, right| {
        (&left.topic_key, &left.locale).cmp(&(&right.topic_key, &right.locale))
    });

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));
    let destination = out_dir.join("builtin_registry.rs");

    let mut generated =
        String::from("pub(crate) const BUILTIN_DOCUMENTS: &[(&str, &str, &str, &str)] = &[\n");

    for bundle in bundles {
        let topic_path = bundle.topic_path.canonicalize()?;
        let questions_path = bundle.questions_path.canonicalize()?;

        generated.push_str(&format!(
            "    ({:?}, {:?}, include_str!({:?}), include_str!({:?})),\n",
            bundle.topic_key,
            bundle.locale,
            topic_path.to_string_lossy(),
            questions_path.to_string_lossy(),
        ));
    }

    generated.push_str("];\n");

    fs::write(destination, generated)?;

    Ok(())
}

fn collect_bundles(
    content_root: &Path,
    directory: &Path,
    bundles: &mut Vec<BundlePaths>,
) -> io::Result<()> {
    if !directory.exists() {
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", directory.display());

    let topic_path = directory.join("topic.json");
    let questions_path = directory.join("questions.json");

    let has_topic = topic_path.is_file();
    let has_questions = questions_path.is_file();

    match (has_topic, has_questions) {
        (true, true) => {
            println!("cargo:rerun-if-changed={}", topic_path.display());
            println!("cargo:rerun-if-changed={}", questions_path.display());

            let locale = directory
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "content locale directory {} has no valid name",
                            directory.display()
                        ),
                    )
                })?;

            if !valid_locale_code(locale) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "content locale directory '{}' must use a lowercase locale code such as en, ru or pt-br",
                        directory.display()
                    ),
                ));
            }

            let topic_directory = directory.parent().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "content locale directory {} must be nested under a topic directory",
                        directory.display()
                    ),
                )
            })?;

            let topic_key = topic_directory
                .strip_prefix(content_root)
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "content topic directory {} is outside {}",
                            topic_directory.display(),
                            content_root.display()
                        ),
                    )
                })?
                .to_string_lossy()
                .replace('\\', "/");

            if topic_key.is_empty() || !topic_key.contains('/') {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "localized content {} must follow <course>/<topic>/<locale>/topic.json",
                        directory.display()
                    ),
                ));
            }

            bundles.push(BundlePaths {
                topic_key,
                locale: locale.to_owned(),
                topic_path,
                questions_path,
            });

            return Ok(());
        }

        (true, false) | (false, true) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "content locale directory {} must contain both topic.json and questions.json",
                    directory.display()
                ),
            ));
        }

        (false, false) => {}
    }

    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();

        if path.is_dir() {
            collect_bundles(content_root, &path, bundles)?;
        }
    }

    Ok(())
}

fn valid_locale_code(value: &str) -> bool {
    if value.is_empty() || value != value.to_ascii_lowercase() {
        return false;
    }

    value.split('-').all(|part| {
        (2..=8).contains(&part.len())
            && part
                .chars()
                .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
    })
}
