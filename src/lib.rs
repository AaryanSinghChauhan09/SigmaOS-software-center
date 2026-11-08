use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppListing {
    pub app_id: String,
    pub title: String,
    pub is_flatpak: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogError {
    EmptyAppId,
    EmptyTitle,
    DuplicateAppId(String),
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAppId => write!(f, "application ID must not be empty"),
            Self::EmptyTitle => write!(f, "application title must not be empty"),
            Self::DuplicateAppId(id) => write!(f, "application ID is already catalogued: {id}"),
        }
    }
}

impl std::error::Error for CatalogError {}

#[derive(Debug, Default)]
pub struct SoftwareCenterCatalog {
    listings: Vec<AppListing>,
}

impl SoftwareCenterCatalog {
    pub fn new() -> Self { Self::default() }
    pub fn add_listing(&mut self, id: &str, title: &str, flatpak: bool) -> Result<(), CatalogError> {
        let id = id.trim();
        let title = title.trim();
        if id.is_empty() {
            return Err(CatalogError::EmptyAppId);
        }
        if title.is_empty() {
            return Err(CatalogError::EmptyTitle);
        }
        if self.listings.iter().any(|listing| listing.app_id == id) {
            return Err(CatalogError::DuplicateAppId(id.to_string()));
        }
        self.listings.push(AppListing {
            app_id: id.to_string(),
            title: title.to_string(),
            is_flatpak: flatpak,
        });
        Ok(())
    }

    /// Query an exact application ID.
    pub fn get(&self, id: &str) -> Option<&AppListing> {
        self.listings.iter().find(|listing| listing.app_id == id)
    }

    /// Search titles by a case-insensitive substring.
    pub fn search_title(&self, query: &str) -> Vec<&AppListing> {
        let query = query.trim().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }
        self.listings
            .iter()
            .filter(|listing| listing.title.to_lowercase().contains(&query))
            .collect()
    }

    /// Iterate over Flatpak-origin listings without exposing mutable storage.
    pub fn flatpak_listings(&self) -> impl Iterator<Item = &AppListing> {
        self.listings.iter().filter(|listing| listing.is_flatpak)
    }

    /// Remove a listing by its exact application ID.
    pub fn remove(&mut self, id: &str) -> Option<AppListing> {
        let index = self.listings.iter().position(|listing| listing.app_id == id)?;
        Some(self.listings.remove(index))
    }

    pub fn count(&self) -> usize { self.listings.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_software_center_listings() {
        let mut catalog = SoftwareCenterCatalog::new();
        catalog.add_listing("org.gimp.GIMP", "GNU Image Manipulation Program", true).unwrap();
        assert_eq!(catalog.count(), 1);
    }
}
