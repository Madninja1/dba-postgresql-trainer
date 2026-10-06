use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

fn main() -> io::Result<()> {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));

    let content_root = manifest_dir.join("../../course-content");

    let mut bundles = Vec::new();

    collect_bundles(&content_root, &mut bundles)?;

    bundles.sort_by(|(left, _), (right, _)| left.cmp(right));

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));

    let destination = out_dir.join("builtin_registry.rs");

    let mut generated = String::from("pub(crate) const BUILTIN_DOCUMENTS: &[(&str, &str)] = &[\n");

    for (topic_path, questions_path) in bundles {
        let topic_path = topic_path.canonicalize()?;

        let questions_path = questions_path.canonicalize()?;

        generated.push_str(&format!(
            "    (include_str!({:?}), include_str!({:?})),\n",
            topic_path.to_string_lossy(),
            questions_path.to_string_lossy(),
        ));
    }

    generated.push_str("];\n");

    fs::write(destination, generated)?;

    Ok(())
}

fn collect_bundles(directory: &Path, bundles: &mut Vec<(PathBuf, PathBuf)>) -> io::Result<()> {
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

            bundles.push((topic_path, questions_path));

            return Ok(());
        }

        (true, false) | (false, true) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "content directory {} must contain both topic.json and questions.json",
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
            collect_bundles(&path, bundles)?;
        }
    }

    Ok(())
}
