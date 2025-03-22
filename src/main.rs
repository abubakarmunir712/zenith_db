mod storage;
use storage::page::Page;

fn main() {
    let _page: Result<Page, String> = Page::new(false, Vec::new());
    if let Err(p) = _page {
        println!("{p}");
    }
}
