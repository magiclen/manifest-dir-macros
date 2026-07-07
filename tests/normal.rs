use std::path::PathBuf;

use manifest_dir_macros::{
    directory_relative_path, exist_relative_path, file_relative_path, get_extension, get_file_name,
    get_file_stem,
};

fn manifest_path(components: &[&str]) -> String {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    for component in components {
        path.push(component);
    }

    path.to_str().unwrap().to_owned()
}

#[test]
fn resolves_existing_relative_paths_with_multiple_components() {
    assert_eq!(
        exist_relative_path!("src", "join_builder.rs"),
        manifest_path(&["src", "join_builder.rs"])
    );
    assert_eq!(
        file_relative_path!("src", "join_builder.rs"),
        manifest_path(&["src", "join_builder.rs"])
    );
    assert_eq!(directory_relative_path!("src"), manifest_path(&["src"]));
}

#[test]
fn gets_path_components_and_default_values() {
    assert_eq!(get_file_name!("src", "join_builder.rs"), "join_builder.rs");
    assert_eq!(get_file_stem!("src", "join_builder.rs"), "join_builder");
    assert_eq!(get_extension!("src", "join_builder.rs"), "rs");
    assert_eq!(get_extension!(default = "fallback", "src", "join_builder"), "fallback");
}

#[cfg(feature = "tuple")]
#[test]
fn tuple_feature_accepts_expr_matchers() {
    macro_rules! make_path {
        ($path:expr) => {
            manifest_dir_macros::path!("src", $path)
        };
    }

    assert_eq!(make_path!(("join_builder.rs",)), manifest_path(&["src", "join_builder.rs"]));
}

#[cfg(feature = "mime_guess")]
#[test]
fn mime_guess_feature_uses_the_last_component_extension() {
    assert_eq!(manifest_dir_macros::mime_guess!("public", "index.html"), "text/html");
    assert_eq!(
        manifest_dir_macros::mime_guess!(default = "application/octet-stream", "README"),
        "application/octet-stream"
    );
}
