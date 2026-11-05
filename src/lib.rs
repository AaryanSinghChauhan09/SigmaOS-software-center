#[derive(Debug, Clone)]
pub struct AppListing {
    pub app_id: String,
    pub title: String,
    pub is_flatpak: bool,
}

#[derive(Debug, Default)]
pub struct SoftwareCenterCatalog {
    listings: Vec<AppListing>,
}

impl SoftwareCenterCatalog {
    pub fn new() -> Self { Self::default() }
    pub fn add_listing(&mut self, id: &str, title: &str, flatpak: bool) {
        self.listings.push(AppListing {
            app_id: id.to_string(),
            title: title.to_string(),
            is_flatpak: flatpak,
        });
    }
    pub fn count(&self) -> usize { self.listings.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_software_center_listings() {
        let mut catalog = SoftwareCenterCatalog::new();
        catalog.add_listing("org.gimp.GIMP", "GNU Image Manipulation Program", true);
        assert_eq!(catalog.count(), 1);
    }
}
