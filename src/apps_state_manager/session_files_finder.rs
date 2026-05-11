use ini::Ini;
use std::path::PathBuf;

pub fn get_firefox_base_dir() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        dirs::home_dir().map(|mut p| {
            p.push(".mozilla");
            p.push("firefox");
            p
        })
    }

    #[cfg(not(target_os = "linux"))]
    {
        unimplemented!("Currently only Linux is supported")
    }
}

pub fn get_active_session_path() -> Option<PathBuf> {
    let base_dir = get_firefox_base_dir()?;
    let conf = Ini::load_from_file(base_dir.join("profiles.ini")).ok()?;

    let profile_dir = find_install_default(&conf).or_else(|| find_legacy_default(&conf))?;

    Some(
        base_dir
            .join(profile_dir)
            .join("sessionstore-backups")
            .join("recovery.jsonlz4"),
    )
}

fn find_install_default(conf: &Ini) -> Option<String> {
    conf.iter()
        .filter_map(|(sec, prop)| sec.zip(Some(prop)))
        .find(|(sec, _)| sec.starts_with("Install"))
        .and_then(|(_, prop)| prop.get("Default"))
        .map(str::to_string)
}

fn find_legacy_default(conf: &Ini) -> Option<String> {
    conf.iter()
        .filter_map(|(sec, prop)| sec.zip(Some(prop)))
        .find(|(sec, prop)| sec.starts_with("Profile") && prop.get("Default") == Some("1"))
        .and_then(|(_, prop)| prop.get("Path"))
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_ini(sections: &[(&str, &[(&str, &str)])]) -> Ini {
        let mut conf = Ini::new();
        for &(section, props) in sections {
            for (key, value) in props {
                conf.set_to(Some(section), key.to_string(), value.to_string());
            }
        }
        conf
    }

    #[test]
    fn returns_default_from_install_section() {
        let conf = make_ini(&[(
            "Install4F96D1932A9F858E",
            &[("Default", "myprofile.default")],
        )]);

        assert_eq!(
            find_install_default(&conf),
            Some("myprofile.default".to_string())
        );
    }

    #[test]
    fn ignores_non_install_sections() {
        let conf = make_ini(&[
            ("Profile0", &[("Default", "should-be-ignored")]),
            ("General", &[("Default", "also-ignored")]),
            ("Install123", &[("Default", "correct-profile")]),
        ]);

        assert_eq!(
            find_install_default(&conf),
            Some("correct-profile".to_string())
        );
    }

    #[test]
    fn returns_none_when_no_install_section() {
        let conf = make_ini(&[
            ("Profile0", &[("Path", "some.path"), ("Default", "1")]),
            ("General", &[("StartWithLastProfile", "1")]),
        ]);

        assert_eq!(find_install_default(&conf), None);
    }

    #[test]
    fn returns_none_when_install_section_has_no_default_key() {
        let conf = make_ini(&[("Install4F96D1932A9F858E", &[("Locked", "1")])]);

        assert_eq!(find_install_default(&conf), None);
    }

    #[test]
    fn picks_first_install_section_if_multiple() {
        let conf = make_ini(&[
            ("InstallAAA", &[("Default", "first-profile")]),
            ("InstallBBB", &[("Default", "second-profile")]),
        ]);

        assert_eq!(
            find_install_default(&conf),
            Some("first-profile".to_string())
        );
    }

    #[test]
    fn empty_ini_returns_none() {
        let conf = Ini::new();
        assert_eq!(find_install_default(&conf), None);
    }
}
