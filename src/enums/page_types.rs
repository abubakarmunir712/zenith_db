use crate::configs::config::Config::{
    CATLOG_PAGE_SIZE, FSM_PAGE_SIZE, INDEX_PAGE_SIZE, PAGE_SIZE, REF_PAGE_SIZE,
};
use crate::storage::page::page::Page;
pub enum PageType {
    DataPage,
    FsmPage,
    CatlogPage,
    IndexPage,
    RefPage,
}

impl PageType {
    pub fn size_in_bytes(&self) -> u64 {
        let size: u64 = match self {
            PageType::DataPage => PAGE_SIZE as u64,
            PageType::IndexPage => INDEX_PAGE_SIZE as u64,
            PageType::FsmPage => FSM_PAGE_SIZE as u64,
            PageType::CatlogPage => CATLOG_PAGE_SIZE as u64,
            PageType::RefPage => REF_PAGE_SIZE as u64,
        };
        size
    }
}
