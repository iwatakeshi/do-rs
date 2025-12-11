//! Pagination utilities for navigating DigitalOcean API list responses.
//!
//! This module provides helpers for working with paginated API responses,
//! making it easier to iterate through large collections of resources.
//!
//! # Example
//!
//! ```rust,ignore
//! use digitalocean::pagination::PaginatedResponse;
//! use digitalocean::apis::reserved_ips_api::reserved_ips_list;
//!
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let configuration = todo!();
//!     // Fetch first page
//!     let response = reserved_ips_list(&configuration, Some(20), Some(1)).await?;
//!
//!     // Check pagination info
//!     println!("Total items: {}", response.total());
//!     println!("Has next page: {}", response.has_next_page());
//!
//!     // Get next page number if available
//!     if let Some(next_page) = response.next_page_number() {
//!         let next_response = reserved_ips_list(&configuration, Some(20), Some(next_page)).await?;
//!     }
//!     Ok(())
//! }
//! ```

use crate::models::{PageLinks, PageLinksPages, MetaProperties};
use url::Url;

/// Trait for paginated API responses.
///
/// Implement this trait on response types that contain pagination metadata
/// to get convenient pagination helper methods.
pub trait PaginatedResponse {
    /// Returns the total number of items across all pages.
    fn total(&self) -> i32;

    /// Returns the pagination links, if present.
    fn page_links(&self) -> Option<&PageLinksPages>;

    /// Returns `true` if there is a next page of results.
    fn has_next_page(&self) -> bool {
        self.page_links()
            .map(|p| p.next.is_some())
            .unwrap_or(false)
    }

    /// Returns `true` if there is a previous page of results.
    fn has_prev_page(&self) -> bool {
        self.page_links()
            .map(|p| p.prev.is_some())
            .unwrap_or(false)
    }

    /// Returns `true` if this is the first page.
    fn is_first_page(&self) -> bool {
        !self.has_prev_page()
    }

    /// Returns `true` if this is the last page.
    fn is_last_page(&self) -> bool {
        !self.has_next_page()
    }

    /// Returns the URL for the next page, if available.
    fn next_page_url(&self) -> Option<&str> {
        self.page_links().and_then(|p| p.next.as_deref())
    }

    /// Returns the URL for the previous page, if available.
    fn prev_page_url(&self) -> Option<&str> {
        self.page_links().and_then(|p| p.prev.as_deref())
    }

    /// Returns the URL for the first page, if available.
    fn first_page_url(&self) -> Option<&str> {
        self.page_links().and_then(|p| p.first.as_deref())
    }

    /// Returns the URL for the last page, if available.
    fn last_page_url(&self) -> Option<&str> {
        self.page_links().and_then(|p| p.last.as_deref())
    }

    /// Extracts and returns the next page number from the pagination URL.
    ///
    /// Returns `None` if there is no next page or if the page number cannot be parsed.
    fn next_page_number(&self) -> Option<i32> {
        self.next_page_url().and_then(extract_page_number)
    }

    /// Extracts and returns the previous page number from the pagination URL.
    ///
    /// Returns `None` if there is no previous page or if the page number cannot be parsed.
    fn prev_page_number(&self) -> Option<i32> {
        self.prev_page_url().and_then(extract_page_number)
    }

    /// Extracts and returns the last page number from the pagination URL.
    ///
    /// This can be used to determine the total number of pages.
    /// Returns `None` if there is no last page URL or if the page number cannot be parsed.
    fn last_page_number(&self) -> Option<i32> {
        self.last_page_url().and_then(extract_page_number)
    }
}

/// Extracts the page number from a DigitalOcean API pagination URL.
///
/// # Example
///
/// ```rust
/// use digitalocean::pagination::extract_page_number;
///
/// let url = "https://api.digitalocean.com/v2/droplets?page=3&per_page=20";
/// assert_eq!(extract_page_number(url), Some(3));
///
/// let url_without_page = "https://api.digitalocean.com/v2/droplets";
/// assert_eq!(extract_page_number(url_without_page), None);
/// ```
pub fn extract_page_number(url: &str) -> Option<i32> {
    Url::parse(url)
        .ok()?
        .query_pairs()
        .find(|(key, _)| key == "page")
        .and_then(|(_, value)| value.parse().ok())
}

/// Extracts the per_page value from a DigitalOcean API pagination URL.
///
/// # Example
///
/// ```rust
/// use digitalocean::pagination::extract_per_page;
///
/// let url = "https://api.digitalocean.com/v2/droplets?page=3&per_page=50";
/// assert_eq!(extract_per_page(url), Some(50));
/// ```
pub fn extract_per_page(url: &str) -> Option<i32> {
    Url::parse(url)
        .ok()?
        .query_pairs()
        .find(|(key, _)| key == "per_page")
        .and_then(|(_, value)| value.parse().ok())
}

/// Helper struct for building pagination parameters.
///
/// # Example
///
/// ```rust
/// use digitalocean::pagination::PageRequest;
///
/// let page = PageRequest::new()
///     .with_page(2)
///     .with_per_page(50);
///
/// assert_eq!(page.page(), Some(2));
/// assert_eq!(page.per_page(), Some(50));
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub struct PageRequest {
    page: Option<i32>,
    per_page: Option<i32>,
}

impl PageRequest {
    /// Creates a new `PageRequest` with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a `PageRequest` for the first page with the specified page size.
    pub fn first(per_page: i32) -> Self {
        Self {
            page: Some(1),
            per_page: Some(per_page),
        }
    }

    /// Sets the page number.
    pub fn with_page(mut self, page: i32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of items per page.
    ///
    /// The DigitalOcean API supports a maximum of 200 items per page.
    pub fn with_per_page(mut self, per_page: i32) -> Self {
        self.per_page = Some(per_page.min(200));
        self
    }

    /// Returns the page number, if set.
    pub fn page(&self) -> Option<i32> {
        self.page
    }

    /// Returns the per_page value, if set.
    pub fn per_page(&self) -> Option<i32> {
        self.per_page
    }

    /// Creates a `PageRequest` for the next page, keeping the same page size.
    pub fn next(&self) -> Self {
        Self {
            page: self.page.map(|p| p + 1),
            per_page: self.per_page,
        }
    }

    /// Creates a `PageRequest` for the previous page, keeping the same page size.
    ///
    /// Returns `None` if already on the first page or page is not set.
    pub fn prev(&self) -> Option<Self> {
        match self.page {
            Some(p) if p > 1 => Some(Self {
                page: Some(p - 1),
                per_page: self.per_page,
            }),
            _ => None,
        }
    }
}

/// Macro to implement `PaginatedResponse` for generated response types.
///
/// This macro reduces boilerplate when implementing the trait for the many
/// paginated response types in the API.
///
/// # Example
///
/// ```rust,ignore
/// impl_paginated_response!(ReservedIpsList200Response);
/// impl_paginated_response!(DropletsList200Response);
/// ```
#[macro_export]
macro_rules! impl_paginated_response {
    ($response_type:ty) => {
        impl $crate::pagination::PaginatedResponse for $response_type {
            fn total(&self) -> i32 {
                self.meta.total.unwrap_or(0)
            }

            fn page_links(&self) -> Option<&$crate::models::PageLinksPages> {
                self.links
                    .as_ref()
                    .and_then(|l| l.pages.as_ref())
                    .and_then(|p| p.as_ref())
                    .map(|b| b.as_ref())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_page_number() {
        assert_eq!(
            extract_page_number("https://api.digitalocean.com/v2/droplets?page=3"),
            Some(3)
        );
        assert_eq!(
            extract_page_number("https://api.digitalocean.com/v2/droplets?page=1&per_page=20"),
            Some(1)
        );
        assert_eq!(
            extract_page_number("https://api.digitalocean.com/v2/droplets"),
            None
        );
        assert_eq!(extract_page_number("invalid-url"), None);
    }

    #[test]
    fn test_extract_per_page() {
        assert_eq!(
            extract_per_page("https://api.digitalocean.com/v2/droplets?per_page=50"),
            Some(50)
        );
        assert_eq!(
            extract_per_page("https://api.digitalocean.com/v2/droplets?page=1&per_page=100"),
            Some(100)
        );
        assert_eq!(
            extract_per_page("https://api.digitalocean.com/v2/droplets"),
            None
        );
    }

    #[test]
    fn test_page_request() {
        let page = PageRequest::new().with_page(2).with_per_page(50);
        assert_eq!(page.page(), Some(2));
        assert_eq!(page.per_page(), Some(50));

        let next = page.next();
        assert_eq!(next.page(), Some(3));
        assert_eq!(next.per_page(), Some(50));

        let prev = page.prev().unwrap();
        assert_eq!(prev.page(), Some(1));
    }

    #[test]
    fn test_page_request_first() {
        let page = PageRequest::first(100);
        assert_eq!(page.page(), Some(1));
        assert_eq!(page.per_page(), Some(100));
    }

    #[test]
    fn test_page_request_max_per_page() {
        let page = PageRequest::new().with_per_page(500);
        assert_eq!(page.per_page(), Some(200)); // Capped at 200
    }

    #[test]
    fn test_page_request_prev_at_first_page() {
        let page = PageRequest::new().with_page(1);
        assert!(page.prev().is_none());
    }
}
