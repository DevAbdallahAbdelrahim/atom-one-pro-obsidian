use grass::{from_path, Options};
use std::path::Path;

#[test]
fn test_main_scss_compilation() {
    let entry_file = Path::new("src").join("scss").join("main.scss");
    let options = Options::default();

    assert!(
        entry_file.exists(),
        "❌ Entry file src/scss/main.scss does not exist!"
    );

    let result = from_path(&entry_file, &options);
    assert!(
        result.is_ok(),
        "❌ SCSS Compilation failed: {:?}",
        result.err()
    );

    let css = result.unwrap();

    assert!(
        css.contains("--atom-red"),
        "❌ Compiled CSS is missing core Atom One variables!"
    );
    assert!(
        css.contains(".workspace-tab-header"),
        "❌ Compiled CSS is missing tab components styling!"
    );
}
