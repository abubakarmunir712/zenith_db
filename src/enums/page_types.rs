use crate::configs::config::Config::{FSM_PAGE_SIZE, INDEX_PAGE_SIZE, PAGE_SIZE};

pub enum PageType {
    DataPage,
    FsmPage,
    CatlogPage,
    IndexPage,
}

impl PageType {
    pub fn size_in_bytes(&self) -> u64 {
        let size: u64 = match self {
            PageType::DataPage => PAGE_SIZE as u64,
            PageType::IndexPage => INDEX_PAGE_SIZE as u64,
            PageType::FsmPage => FSM_PAGE_SIZE as u64,
            PageType::CatlogPage => 0,
        };
        size
    }
}
