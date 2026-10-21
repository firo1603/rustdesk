pub const OWNER: &str = "firo1603";
pub const REPO: &str = "rustdesk";

pub fn latest_release_api_url() -> String {
    format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/latest")
}

pub fn release_page_url(tag: &str) -> String {
    format!("https://github.com/{OWNER}/{REPO}/releases/tag/{tag}")
}

pub fn version_from_tag(tag: &str) -> &str {
    tag.strip_prefix("Version")
        .or_else(|| tag.strip_prefix('v'))
        .unwrap_or(tag)
}

#[cfg(test)]
mod tests {
    use super::{release_page_url, version_from_tag};

    #[test]
    fn release_tags_keep_their_download_path_and_use_numeric_versions() {
        for tag in ["Version1.5.0", "v1.5.0", "1.5.0"] {
            assert_eq!(version_from_tag(tag), "1.5.0");
            assert_eq!(
                release_page_url(tag),
                format!("https://github.com/firo1603/rustdesk/releases/tag/{tag}")
            );
        }
        assert_eq!(version_from_tag("Version1.5.0-1"), "1.5.0-1");
    }
}
