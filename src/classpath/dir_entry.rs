struct DirEntry {
    abs_dir: String
}

impl DirEntry {
    fn new(path: &str) -> DirEntry {
        Self { abs_dir: path.to_string() }
    }
}

impl Entry for DirEntry {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, Arc<dyn Entry>)> {
        let pb = Path::new(&self.abs_dir).join(class_name);
        let path = pb.as_path();
        if path.is_file() {
            let file = File::open(path).unwrap();
            Some((file_util::read_file(&file), self))
        } else {
            None
        }
    }
}

impl ToString for DirEntry {
    fn to_string(&self) -> String {
        self.abs_dir.to_string()
    }
}