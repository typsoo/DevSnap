#[test]
#[ignore]
fn firefox_base_dir_exists() {
    let base_dir = get_firefox_base_dir().expect("failed to determine Firefox base directory");

    assert!(
        base_dir.exists(),
        "Firefox directory not found: {}",
        base_dir.display()
    );
}

#[test]
#[ignore]
fn profiles_ini_exists_and_is_parseable() {
    let base_dir = get_firefox_base_dir().expect("failed to determine Firefox base directory");

    let ini_path = base_dir.join("profiles.ini");

    assert!(
        ini_path.exists(),
        "profiles.ini not found at: {}",
        ini_path.display()
    );

    let conf = ini::Ini::load_from_file(&ini_path).expect("failed to parse profiles.ini");

    assert!(conf.iter().count() > 0, "profiles.ini is empty");
}

#[test]
#[ignore]
fn active_session_path_resolves() {
    let path = get_active_session_path()
        .expect("failed to resolve session path — no profile found in profiles.ini");

    println!("resolved session path: {}", path.display());

    assert!(
        path.exists(),
        "session file does not exist: {}\n\
         (is Firefox running? recovery.jsonlz4 is only created during an active session)",
        path.display()
    );
}

#[test]
#[ignore]
fn session_file_is_not_empty() {
    let path = get_active_session_path().expect("failed to resolve session path");

    let metadata = std::fs::metadata(&path).expect("failed to read session file metadata");

    assert!(
        metadata.len() > 0,
        "session file is empty: {}",
        path.display()
    );
}
