use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
struct BundlePaths {
    content_key: String,
    locale: String,
    first_path: PathBuf,
    second_path: PathBuf,
}

fn main() -> io::Result<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));

    let content_root = manifest_dir.join("../../course-content");

    let mut topics = Vec::new();
    let mut labs = Vec::new();

    collect_documents(&content_root, &content_root, &mut topics, &mut labs)?;

    topics.sort_by(|left, right| {
        (&left.content_key, &left.locale).cmp(&(&right.content_key, &right.locale))
    });
    labs.sort_by(|left, right| {
        (&left.content_key, &left.locale).cmp(&(&right.content_key, &right.locale))
    });

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));
    let destination = out_dir.join("builtin_registry.rs");

    let mut generated =
        String::from("pub(crate) const BUILTIN_DOCUMENTS: &[(&str, &str, &str, &str)] = &[\n");

    for bundle in topics {
        let first_path = bundle.first_path.canonicalize()?;
        let second_path = bundle.second_path.canonicalize()?;

        generated.push_str(&format!(
            "    ({:?}, {:?}, include_str!({:?}), include_str!({:?})),\n",
            bundle.content_key,
            bundle.locale,
            first_path.to_string_lossy(),
            second_path.to_string_lossy(),
        ));
    }

    generated.push_str("];\n\n");
    generated.push_str(
        "pub(crate) const BUILTIN_LAB_DOCUMENTS: &[(&str, &str, &str, &str)] = &[\n",
    );

    for bundle in labs {
        let first_path = bundle.first_path.canonicalize()?;
        let second_path = bundle.second_path.canonicalize()?;

        generated.push_str(&format!(
            "    ({:?}, {:?}, include_str!({:?}), include_str!({:?})),\n",
            bundle.content_key,
            bundle.locale,
            first_path.to_string_lossy(),
            second_path.to_string_lossy(),
        ));
    }

    generated.push_str("];\n");

    fs::write(destination, generated)?;

    Ok(())
}

fn collect_documents(
    content_root: &Path,
    directory: &Path,
    topics: &mut Vec<BundlePaths>,
    labs: &mut Vec<BundlePaths>,
) -> io::Result<()> {
    if !directory.exists() {
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", directory.display());

    let topic_path = directory.join("topic.json");
    let questions_path = directory.join("questions.json");
    let lab_path = directory.join("lab.json");
    let lab_markdown_path = directory.join("lab.md");

    let has_topic = topic_path.is_file();
    let has_questions = questions_path.is_file();
    let has_lab = lab_path.is_file();
    let has_lab_markdown = lab_markdown_path.is_file();

    if has_topic || has_questions {
        if !(has_topic && has_questions) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "content locale directory {} must contain both topic.json and questions.json",
                    directory.display()
                ),
            ));
        }

        register_pair(
            content_root,
            directory,
            topic_path,
            questions_path,
            topics,
            "topic",
        )?;
        return Ok(());
    }

    if has_lab || has_lab_markdown {
        if !(has_lab && has_lab_markdown) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "lab locale directory {} must contain both lab.json and lab.md",
                    directory.display()
                ),
            ));
        }

        register_pair(
            content_root,
            directory,
            lab_path,
            lab_markdown_path,
            labs,
            "lab",
        )?;
        return Ok(());
    }

    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();

        if path.is_dir() {
            collect_documents(content_root, &path, topics, labs)?;
        }
    }

    Ok(())
}

fn register_pair(
    content_root: &Path,
    directory: &Path,
    first_path: PathBuf,
    second_path: PathBuf,
    output: &mut Vec<BundlePaths>,
    content_type: &str,
) -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", first_path.display());
    println!("cargo:rerun-if-changed={}", second_path.display());

    let locale = directory
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("content locale directory {} has no valid name", directory.display()),
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

    let content_directory = directory.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "content locale directory {} must be nested under a {} directory",
                directory.display(), content_type
            ),
        )
    })?;

    let content_key = content_directory
        .strip_prefix(content_root)
        .map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "content directory {} is outside {}",
                    content_directory.display(),
                    content_root.display()
                ),
            )
        })?
        .to_string_lossy()
        .replace('\\', "/");

    if content_key.is_empty() || !content_key.contains('/') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "localized {content_type} content {} is not nested deeply enough under course-content",
                directory.display()
            ),
        ));
    }

    output.push(BundlePaths {
        content_key,
        locale: locale.to_owned(),
        first_path,
        second_path,
    });

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
