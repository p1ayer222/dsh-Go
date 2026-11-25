//! Pinned pre-iOS-17 Developer disk images from the DeveloperDiskImage mirror.
pub(crate) const MIRROR_COMMIT: &str = "e2aaeac89ca8fa56c2fa1f144d182994e07a525b";

pub(crate) struct LegacyPin {
    pub(crate) release: &'static str,
    pub(crate) image_bytes: usize,
    pub(crate) image_sha256: &'static str,
    pub(crate) signature_bytes: usize,
    pub(crate) signature_sha256: &'static str,
}

pub(crate) const RELEASES: &[LegacyPin] = &[
    LegacyPin { release: "16.0", image_bytes: 8_464_365, image_sha256: "429d2d05b7c9093f214b117a26571fa7d9596c5ff03e7983b70caff743f0afb1", signature_bytes: 128, signature_sha256: "d30ae9c12874254c5cb453abb7c412c6796913e5533de6b68f1b28f435981100" },
    LegacyPin { release: "16.1", image_bytes: 8_448_885, image_sha256: "086bf1c1a278615e60dd5ce2d2ff55a21996bbdef3dc67df20e209e7dcd48558", signature_bytes: 128, signature_sha256: "54b0ad8113231b1e11faebfdae4b347b482ed12dbe8caf155d25b17e74b1a60e" },
    LegacyPin { release: "16.2", image_bytes: 8_466_283, image_sha256: "d30f6d1274e1300910cd22c1104136006f0c55e28110b16cf20ed1ec48fad0e5", signature_bytes: 128, signature_sha256: "247aa8dd50dea880437a1fabfda21552971f88d39384aca6f13d2c0eba7bdac0" },
    LegacyPin { release: "16.3", image_bytes: 8_466_283, image_sha256: "d30f6d1274e1300910cd22c1104136006f0c55e28110b16cf20ed1ec48fad0e5", signature_bytes: 128, signature_sha256: "247aa8dd50dea880437a1fabfda21552971f88d39384aca6f13d2c0eba7bdac0" },
    LegacyPin { release: "16.4", image_bytes: 7_432_520, image_sha256: "6e4449ff8c751b4432934ade838382c79330b096b865eebabbc1fa42ead9c185", signature_bytes: 128, signature_sha256: "4e82ee8aa75c5ecdd320e075b7d9e4ea2fc1915d0bb776007e8dbc5728ca5d86" },
    LegacyPin { release: "16.5", image_bytes: 7_427_565, image_sha256: "99fcf9a7300ab2504a2b9a79f210702f2f6d6eb056053fc8517b80a6a3e18a16", signature_bytes: 128, signature_sha256: "355169a9753c23fff95f6ebc1e39a433f653a9e0e059580642d92629c1ac0071" },
    LegacyPin { release: "16.6", image_bytes: 7_432_520, image_sha256: "6e4449ff8c751b4432934ade838382c79330b096b865eebabbc1fa42ead9c185", signature_bytes: 128, signature_sha256: "4e82ee8aa75c5ecdd320e075b7d9e4ea2fc1915d0bb776007e8dbc5728ca5d86" },
    LegacyPin { release: "16.7", image_bytes: 7_432_520, image_sha256: "6e4449ff8c751b4432934ade838382c79330b096b865eebabbc1fa42ead9c185", signature_bytes: 128, signature_sha256: "4e82ee8aa75c5ecdd320e075b7d9e4ea2fc1915d0bb776007e8dbc5728ca5d86" },
];

pub(crate) fn catalog_id(release: &str) -> String {
    format!("ddi-legacy-{MIRROR_COMMIT}-{release}-v1")
}

pub(crate) fn pin(release: &str) -> Option<&'static LegacyPin> {
    RELEASES.iter().find(|row| row.release == release)
}

pub(crate) const CACHE_REQUIRED: &[(&str, &str)] = &[
    ("16.0", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.0"),
    ("16.1", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.1"),
    ("16.2", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.2"),
    ("16.3", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.3"),
    ("16.4", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.4"),
    ("16.5", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.5"),
    ("16.6", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.6"),
    ("16.7", "ERR_PHONE_DDI_CACHE_REQUIRED:legacy-16.7"),
];

/// pymobiledevice3 uses `major.minor` from ProductVersion (e.g. 16.6.1 → 16.6).
pub(crate) fn release_label(product_version: &str) -> Option<&'static str> {
    let mut parts = product_version.split('.');
    let major = parts.next()?.parse::<u32>().ok()?;
    let minor = parts.next()?.parse::<u32>().ok()?;
    if major != 16 { return None; }
    match minor {
        0 => Some("16.0"),
        1 => Some("16.1"),
        2 => Some("16.2"),
        3 => Some("16.3"),
        4 => Some("16.4"),
        5 => Some("16.5"),
        6 => Some("16.6"),
        7 => Some("16.7"),
        _ => None,
    }
    .filter(|label| pin(label).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_labels_match_mirror_rows() {
        assert_eq!(release_label("16.6.1"), Some("16.6"));
        assert_eq!(release_label("16.0"), Some("16.0"));
        assert!(release_label("17.0").is_none());
        assert!(release_label("16.8").is_none());
    }
}