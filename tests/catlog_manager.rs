use ZenithDB::{
    configs::config::Config::CATLOG_PAGE_SIZE,
    enums::types::{
        catalog_types::{CatalogData, CatalogType},
        page_types::PageType,
    },
    storage::{
        buffer::catalog_buffer::CatalogBuffer,
        catalog::{catalog_manager::CatalogManager, maps::table_map::TableMap},
        io::file_io::IOEngine,
    },
};

#[test]
fn test_catlog_manager() {
    let cb = CatalogBuffer::new();
    let db_name = "catalog_manger_test_db";
    let table_name = "test_table";
    IOEngine::create_db(db_name).unwrap();
    let table_map = TableMap::new();
    let mut buffer = [0; CATLOG_PAGE_SIZE as usize];
    table_map.serialize(&mut buffer);
    IOEngine::append_page(db_name, db_name, &buffer, PageType::CatlogPage).unwrap();
    let page = cb
        .get_page(db_name, CatalogType::TableMap, 0, true)
        .unwrap();
    let mut page = page.write().unwrap();
    if let CatalogData::TableMap(m) = &mut *page {
        let oid = CatalogManager::create_table(table_name, m).unwrap();
        assert_eq!(oid, 1);
        let oid = CatalogManager::create_table("abc", m).unwrap();
        assert_eq!(oid, 2);
        let oid = CatalogManager::delete_table(table_name, m);
        assert_eq!(oid, 1);
        let oid = CatalogManager::create_table(table_name, m).unwrap();
        assert_eq!(oid, 1);
        let oid = CatalogManager::create_table(
            "12128238123891292918398129398190389280132291839218391208390128391208391290",
            m,
        );
        assert!(oid.is_err());

        assert_eq!(m.no_of_tables(), 2);
    }
    IOEngine::delete_db(db_name).unwrap();
}
