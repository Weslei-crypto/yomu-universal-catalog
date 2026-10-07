#![no_std]

use aidoku::{
    alloc::vec::Vec,
    Listing,
    ListingProvider,
    MangaPageResult,
    Result,
    Source,
    prelude::*,
};

#[derive(Default)]
struct YomuUniversalCatalog;

impl Source for YomuUniversalCatalog {}

impl ListingProvider for YomuUniversalCatalog {
    fn get_manga_list(
        &self,
        _listing: Listing,
        _page: i32,
    ) -> Result<MangaPageResult> {
        Ok(MangaPageResult {
            entries: Vec::new(),
            has_next_page: false,
        })
    }
}

register_source!(
    YomuUniversalCatalog,
    ListingProvider
);
