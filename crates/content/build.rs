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

#[derive(Debug)]
struct PracticeBundlePaths {
    content_key: String,
    locale: String,
    document_path: PathBuf,
    setup_path: PathBuf,
    solution_path: PathBuf,
}

fn main() -> io::Result<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));

    let content_root = manifest_dir.join("../../course-content");

    let mut topics = Vec::new();
    let mut labs = Vec::new();
    let mut practice = Vec::new();

    collect_documents(
        &content_root,
        &content_root,
        &mut topics,
        &mut labs,
        &mut practice,
    )?;

    topics.sort_by(|left, right| {
        (&left.content_key, &left.locale).cmp(&(&right.content_key, &right.locale))
    });
    labs.sort_by(|left, right| {
        (&left.content_key, &left.locale).cmp(&(&right.content_key, &right.locale))
    });
    practice.sort_by(|left, right| {
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
    generated
        .push_str("pub(crate) const BUILTIN_LAB_DOCUMENTS: &[(&str, &str, &str, &str)] = &[\n");

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

    generated.push_str("];\n\n");
    generated.push_str(
        "pub(crate) const BUILTIN_PRACTICE_DOCUMENTS: &[(&str, &str, &str, &str, &str)] = &[\n",
    );

    for bundle in practice {
        let document_path = bundle.document_path.canonicalize()?;
        let setup_path = bundle.setup_path.canonicalize()?;
        let solution_path = bundle.solution_path.canonicalize()?;

        generated.push_str(&format!(
            "    ({:?}, {:?}, include_str!({:?}), include_str!({:?}), include_str!({:?})),\n",
            bundle.content_key,
            bundle.locale,
            document_path.to_string_lossy(),
            setup_path.to_string_lossy(),
            solution_path.to_string_lossy(),
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
    practice: &mut Vec<PracticeBundlePaths>,
) -> io::Result<()> {
    if !directory.exists() {
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", directory.display());

    let topic_path = directory.join("topic.json");
    let questions_path = directory.join("questions.json");
    let lab_path = directory.join("lab.json");
    let lab_markdown_path = directory.join("lab.md");
    let practice_path = directory.join("practice.json");
    let practice_setup_path = directory.join("setup.sql");
    let practice_solution_path = directory.join("solution.sql");

    let has_topic = topic_path.is_file();
    let has_questions = questions_path.is_file();
    let has_lab = lab_path.is_file();
    let has_lab_markdown = lab_markdown_path.is_file();
    let has_practice = practice_path.is_file();
    let has_practice_setup = practice_setup_path.is_file();
    let has_practice_solution = practice_solution_path.is_file();

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

    if has_practice || has_practice_setup || has_practice_solution {
        if !(has_practice && has_practice_setup && has_practice_solution) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "practice locale directory {} must contain practice.json, setup.sql and solution.sql",
                    directory.display()
                ),
            ));
        }

        register_practice(
            content_root,
            directory,
            practice_path,
            practice_setup_path,
            practice_solution_path,
            practice,
        )?;
        return Ok(());
    }

    let mut entries = fs::read_dir(directory)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();

        if path.is_dir() {
            collect_documents(content_root, &path, topics, labs, practice)?;
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

    let (content_key, locale) = localized_content_key(content_root, directory, content_type)?;

    output.push(BundlePaths {
        content_key,
        locale,
        first_path,
        second_path,
    });

    Ok(())
}

fn register_practice(
    content_root: &Path,
    directory: &Path,
    document_path: PathBuf,
    setup_path: PathBuf,
    solution_path: PathBuf,
    output: &mut Vec<PracticeBundlePaths>,
) -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", document_path.display());
    println!("cargo:rerun-if-changed={}", setup_path.display());
    println!("cargo:rerun-if-changed={}", solution_path.display());

    let (content_key, locale) = localized_content_key(content_root, directory, "practice")?;

    output.push(PracticeBundlePaths {
        content_key,
        locale,
        document_path,
        setup_path,
        solution_path,
    });

    Ok(())
}

fn localized_content_key(
    content_root: &Path,
    directory: &Path,
    content_type: &str,
) -> io::Result<(String, String)> {
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

    let content_directory = directory.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "content locale directory {} must be nested under a {} directory",
                directory.display(),
                content_type
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

    Ok((content_key, locale.to_owned()))
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
