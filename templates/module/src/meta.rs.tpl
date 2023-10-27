extern crate alloc;

use alloc::string::String;

use mochi::{
    error::Result,
    mochi_bind,
    structs::meta::{
        DiscoverListings, Meta, Paging, Playlist, PlaylistDetails, SearchFilters, SearchQuery,
    },
};

use super::{{ module.struct_name }};

#[mochi_bind]
impl Meta for {{ module.struct_name }} {
    fn search_filters() -> SearchFilters {
        todo!("not implemented")
    }

    fn search(search_query: SearchQuery) -> Result<Paging<Playlist>> {
        todo!("not implemented")
    }

    fn discover_listings() -> Result<DiscoverListings> {
        todo!("not implemented")
    }

    fn playlist_details(id: String) -> Result<PlaylistDetails> {
        todo!("not implemented")
    }
}