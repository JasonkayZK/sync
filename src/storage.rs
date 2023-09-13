use std::collections::BTreeSet;
use std::sync::OnceLock;

use log::info;
use parking_lot::Mutex;

#[derive(Debug)]
pub struct StorageHandler {
    data: BTreeSet<String>,
}

impl StorageHandler {
    // init global
    pub fn global() -> &'static Mutex<StorageHandler> {
        static STORAGE: OnceLock<Mutex<StorageHandler>> = OnceLock::new();

        STORAGE.get_or_init(|| Mutex::new(StorageHandler::new()))
    }

    fn new() -> Self {
        let d = BTreeSet::new();

        info!("load data success: {:#?}", d);

        Self { data: d }
    }

    pub fn contains(&self, k: &str) -> bool {
        self.data.contains(k)
    }

    pub fn add(&mut self, k: String) {
        self.data.insert(k);
    }

    pub fn remove(&mut self, k: &str) -> bool {
        self.data.remove(k)
    }

    pub fn merge_data(&mut self, data: &mut BTreeSet<String>) {
        self.data.append(data)
    }

    pub fn get_copy_data(&self) -> BTreeSet<String> {
        self.data.iter().map(|x| x.to_string()).collect()
    }

    pub fn print(&self) {
        info!("{:#?}", self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::StorageHandler;

    #[test]
    fn test_main() {
        let s = StorageHandler::global();
        s.lock().print();
        assert_eq!(s.lock().data.len(), 0);

        s.lock().add("4".to_string());
        s.lock().print();
        assert_eq!(s.lock().data.len(), 1);

        s.lock().remove("4");
        s.lock().print();
        assert_eq!(s.lock().data.len(), 0);
    }
}
