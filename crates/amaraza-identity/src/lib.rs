//! amaraza-identity — what makes this build Amaraza PDF rather than PdfCraft: the product name,
//! app id, data folders, portable-mode markers, web addresses and logo. Files taken from
//! storytold/pdfcraft only refer to these constants, so a merge from upstream conflicts on a few
//! lines instead of on every string (docs/amaraza/PLAN.md, AR0.1).
//!
//! No dependencies, so any layer may use it.

#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
// A7 (.agents/rules/00-amaraza.md §9): the threshold, 15, is in this crate's clippy.toml.
#![warn(clippy::cognitive_complexity)]

/// The product name people see: window title, About, installer, shortcuts.
pub const PRODUCT_NAME: &str = "Amaraza PDF";

/// Reverse-DNS application id: the Wayland app id, the `.desktop` file and hicolor icon name, the
/// macOS bundle id and the Flatpak id.
pub const APP_ID: &str = "io.github.pony_svg.AmarazaPDF";

/// The name of the folders that hold settings, logs, crash-recovery autosaves and new digital IDs
/// (`%APPDATA%\AmarazaPDF`, `%LOCALAPPDATA%\AmarazaPDF\Recovery`, `~/Library/Application
/// Support/AmarazaPDF`), and the stem of the installed executable.
pub const FOLDER_NAME: &str = "AmarazaPDF";

/// [`FOLDER_NAME`] lowercased, where the platform's convention is lowercase folders
/// (`~/.local/share/amarazapdf`).
pub const FOLDER_NAME_LOWER: &str = "amarazapdf";

/// The name the installer and the portable zip give the app's executable. Cargo still builds
/// `pdfcraft.exe`; it is renamed only when packaged, which keeps upstream build files untouched.
pub const EXE_NAME: &str = "AmarazaPDF.exe";

/// Files beside the executable that switch on portable mode. Never PdfCraft's markers
/// (`portable.txt`, `PdfCraft.portable`), so neither app picks up the other's portable data.
pub const PORTABLE_MARKERS: &[&str] = &["AmarazaPDF.portable"];

/// The data folder created beside the executable in portable mode.
pub const PORTABLE_DATA_DIR: &str = "AmarazaPDFData";

/// The publisher: the installer's manufacturer and the company in the executable's version
/// information.
pub const MANUFACTURER: &str = "Amaraza";

/// The MSI `UpgradeCode` (packaging/windows/pdfcraft.wxs): fixed for ever, so every Amaraza PDF
/// installer upgrades the previous one. Never PdfCraft's or PDForte's: a shared code would let
/// one app's installer remove the other.
pub const MSI_UPGRADE_CODE: &str = "B023D7AF-E898-4990-9855-B539545099F3";

/// The source repository.
pub const REPOSITORY: &str = "https://github.com/pony-svg/AmarazaPDF-rs";

/// Where releases (the downloads) are listed. An update check only ever offers a page under it.
pub const RELEASES_PAGE: &str = "https://github.com/pony-svg/AmarazaPDF-rs/releases";

/// GitHub's API address for the latest release.
pub const LATEST_RELEASE_API: &str = "https://api.github.com/repos/pony-svg/AmarazaPDF-rs/releases/latest";

/// The project this one is based on. Its brand licence allows crediting it in plain text only:
/// no logos, and nothing that presents Amaraza PDF as an upstream product.
pub const UPSTREAM_NAME: &str = "PdfCraft";

/// The upstream source repository.
pub const UPSTREAM_REPOSITORY: &str = "https://github.com/storytold/pdfcraft";

/// The attribution line for About, the README and NOTICE.
pub const BASED_ON: &str = "Basado en PdfCraft, del equipo ArtCraft";

/// The Amaraza PDF logo (master vector), a trademark of its owner under
/// `LicenseRef-Amaraza-Trademark` (docs/brand/LICENSE-brand.txt), not the code's licence.
pub const LOGO_SVG: &[u8] = include_bytes!("../../../docs/brand/amaraza-logo.svg");

/// The logo as a 512 px PNG: the window icon.
pub const LOGO_PNG: &[u8] = include_bytes!("../../../docs/brand/amaraza-logo.png");

/// The logo as a Windows icon (16–256 px), relative to the repository root: the executable's
/// resource (`apps/pdfcraft/build.rs`) and the installer's icon (`packaging/windows`).
pub const LOGO_ICO_PATH: &str = "docs/brand/amaraza-logo.ico";

#[cfg(test)]
mod tests {
    use super::*;

    /// Names that belong to other apps: upstream (and its former name), its publisher, and the
    /// Python Amaraza PDF (PDForte), whose folders and installer must never be shared.
    const FOREIGN: &[&str] = &["pdfcraft", "printcraft", "artcraft", "storyteller", "storytold", "learning machines", "pdforte"];

    fn own_identifiers() -> Vec<&'static str> {
        let mut ids = vec![
            PRODUCT_NAME,
            APP_ID,
            FOLDER_NAME,
            FOLDER_NAME_LOWER,
            EXE_NAME,
            PORTABLE_DATA_DIR,
            MANUFACTURER,
            REPOSITORY,
            RELEASES_PAGE,
            LATEST_RELEASE_API,
        ];
        ids.extend_from_slice(PORTABLE_MARKERS);
        ids
    }

    #[test]
    fn identifiers_are_never_another_apps() {
        for id in own_identifiers() {
            let lower = id.to_lowercase();
            for foreign in FOREIGN {
                assert!(!lower.contains(foreign), "{id:?} reuses {foreign:?}");
            }
        }
    }

    #[test]
    fn app_id_is_reverse_dns() {
        // Freedesktop and Flatpak: at least three dot-separated elements of [A-Za-z0-9_], none
        // empty or starting with a digit.
        let elements: Vec<&str> = APP_ID.split('.').collect();
        assert!(elements.len() >= 3, "{APP_ID}");
        for e in elements {
            assert!(!e.is_empty() && !e.starts_with(|c: char| c.is_ascii_digit()), "{APP_ID}: element {e:?}");
            assert!(e.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'), "{APP_ID}: element {e:?}");
        }
    }

    #[test]
    fn names_and_addresses_agree() {
        assert_eq!(FOLDER_NAME_LOWER, FOLDER_NAME.to_lowercase());
        assert_eq!(EXE_NAME, format!("{FOLDER_NAME}.exe"));
        assert_eq!(PORTABLE_DATA_DIR, format!("{FOLDER_NAME}Data"));
        assert!(PORTABLE_MARKERS.iter().all(|m| m.starts_with(FOLDER_NAME)), "{PORTABLE_MARKERS:?}");
        assert_eq!(RELEASES_PAGE, format!("{REPOSITORY}/releases"));
        let repo = REPOSITORY.strip_prefix("https://github.com/").unwrap_or_default();
        assert_eq!(LATEST_RELEASE_API, format!("https://api.github.com/repos/{repo}/releases/latest"));
        assert!(BASED_ON.contains(UPSTREAM_NAME), "{BASED_ON}");
    }

    fn repo_file(path: &str) -> String {
        let full = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(path);
        std::fs::read_to_string(&full).unwrap_or_else(|e| panic!("{}: {e}", full.display()))
    }

    #[test]
    fn the_windows_installer_is_amarazas() {
        let wxs = repo_file("packaging/windows/pdfcraft.wxs");
        assert!(wxs.contains(&format!("UpgradeCode=\"{MSI_UPGRADE_CODE}\"")), "UpgradeCode");
        assert!(wxs.contains(&format!("Name=\"{PRODUCT_NAME}\"")), "product name");
        assert!(wxs.contains(&format!("Manufacturer=\"{MANUFACTURER}\"")), "manufacturer");
        assert!(wxs.contains(&format!("Name=\"{EXE_NAME}\"")), "installed executable name");
        assert!(wxs.contains(&format!("Id=\"INSTALLFOLDER\" Name=\"{FOLDER_NAME}\"")), "install folder");
        assert!(wxs.contains(REPOSITORY), "Add/Remove Programs links");
        // PdfCraft's identity: its UpgradeCode and component GUIDs, publisher, ProgId, registered
        // application, links and install folder.
        for upstream in [
            "EA00C367-7270-4C50-9D8B-A7A5C88AD27F",
            "E724C1C6-091F-4E21-A685-80D089FDBE21",
            "DC028192-C233-4F99-9A77-ED458C60DF26",
            "F6948627-32AB-4D78-8865-0FDF417CE442",
            "Learning Machines",
            "PdfCraft.Document",
            "PdfCraft.CreatePdf",
            "Software\\PdfCraft",
            "getartcraft",
            "discord",
            "Name=\"PdfCraft\"",
        ] {
            assert!(!wxs.contains(upstream), "pdfcraft.wxs still has {upstream:?}");
        }
        let package = repo_file("packaging/windows/package.ps1");
        assert!(
            package.contains(EXE_NAME) && package.contains(PORTABLE_MARKERS[0]) && package.contains(LOGO_ICO_PATH.replace('/', "\\").as_str()),
            "package.ps1"
        );
        assert!(!package.contains("'portable.txt'") && !package.contains("app-icon"), "package.ps1 still ships PdfCraft's marker or icon");
        let marker = repo_file(&format!("packaging/windows/{}", PORTABLE_MARKERS[0]));
        assert!(marker.contains(PORTABLE_DATA_DIR) && !marker.contains("PdfCraft"), "portable marker text");
        let test_msi = repo_file("packaging/windows/test-msi.ps1");
        assert!(test_msi.contains(MSI_UPGRADE_CODE) && test_msi.contains(&format!("'{MANUFACTURER}'")), "test-msi.ps1 checks the identity");
    }

    #[test]
    fn the_other_platforms_packages_use_the_app_id() {
        let desktop = repo_file(&format!("packaging/linux/{APP_ID}.desktop"));
        assert!(desktop.contains(&format!("Name={PRODUCT_NAME}\n")) && desktop.contains(&format!("Icon={APP_ID}\n")), "desktop entry");
        let metainfo = repo_file(&format!("packaging/linux/{APP_ID}.metainfo.xml.in"));
        assert!(metainfo.contains(&format!("<id>{APP_ID}</id>")), "AppStream id");
        assert!(metainfo.contains(&format!("<launchable type=\"desktop-id\">{APP_ID}.desktop</launchable>")), "AppStream launchable");
        for manifest in [format!("packaging/linux/flatpak/{APP_ID}.yml"), format!("packaging/linux/flatpak/{APP_ID}.bundle.yml")] {
            assert!(repo_file(&manifest).contains(&format!("id: {APP_ID}\n")), "{manifest}");
        }
        for script in ["packaging/linux/package.sh", "packaging/linux/flatpak-bundle.sh", "packaging/freebsd/package.sh"] {
            assert!(repo_file(script).contains(&format!("APP_ID={APP_ID}\n")), "{script}");
        }
        let plist = repo_file("packaging/macos/Info.plist.in");
        assert!(plist.contains(&format!("<string>{APP_ID}</string>")) && plist.contains(&format!("<string>{PRODUCT_NAME}</string>")), "Info.plist");
        // No packaging file names upstream's id, publisher, site or community, or its icon set.
        let files = files_under(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packaging"));
        let mut checked = 0;
        for (path, text) in &files {
            assert!(!path.to_string_lossy().to_lowercase().contains("storyteller"), "{}", path.display());
            let Some(text) = text else { continue };
            checked += 1;
            for upstream in ["storyteller", "getartcraft", "discord.gg", "learning machines", "assets/app-icon"] {
                assert!(!text.to_lowercase().contains(upstream), "{} still has {upstream:?}", path.display());
            }
        }
        assert!(checked > 10, "only {checked} packaging files read");
    }

    /// Every file under `dir`, recursively, with its text when it is UTF-8.
    fn files_under(dir: std::path::PathBuf) -> Vec<(std::path::PathBuf, Option<String>)> {
        let mut out = Vec::new();
        let mut pending = vec![dir];
        while let Some(dir) = pending.pop() {
            for entry in std::fs::read_dir(&dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    pending.push(path);
                } else {
                    let text = std::fs::read_to_string(&path).ok();
                    out.push((path, text));
                }
            }
        }
        out
    }

    #[test]
    fn the_logo_is_amarazas_in_every_format() {
        assert!(LOGO_SVG.starts_with(b"<svg"), "an SVG document");
        assert!(LOGO_PNG.starts_with(b"\x89PNG\r\n\x1a\n"), "a PNG image");
        let ico = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(LOGO_ICO_PATH);
        let ico = std::fs::read(&ico).unwrap();
        assert!(ico.starts_with(&[0, 0, 1, 0]), "a Windows icon");
        // The brand folder holds only Amaraza's marks: upstream's are gone for good.
        let brand = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/brand");
        for entry in std::fs::read_dir(brand).unwrap() {
            let name = entry.unwrap().file_name().to_string_lossy().to_lowercase();
            assert!(name.starts_with("amaraza-") || name == "license-brand.txt", "{name}");
        }
    }
}
