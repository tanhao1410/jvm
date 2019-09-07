use crate::util::file_util;
use std::path::Path;
use std::option::Option;
use std::string::ToString;
use std::io::{BufReader, Read};

pub trait Entry: ToString {
    fn read_class(&self, className: &str) -> Option<(Vec<u8>, &dyn Entry)>;
}

struct DirEntry<'a> {
    abs_dir: &'a Path
}

struct ZipEntry<'a> {
    abs_path: &'a Path
}

struct CompositeEntry<'a> {
    entrys: Vec<Box<dyn Entry + 'a>>
}

struct WildcardEntry<'a> {
    entry: CompositeEntry<'a>
}

pub fn new_entry(path: &str) -> Box<dyn Entry + '_> {
    if path.contains(if cfg!(windows) { ';' } else { ':' }) {
        Box::new(CompositeEntry::new(path))
    } else if path.ends_with("*") {
        Box::new(WildcardEntry::new(path))
    } else if path.ends_with(".jar") || path.ends_with(".JAR") || path.ends_with(".zip") || path.ends_with(".ZIP") {
        Box::new(ZipEntry::new(path))
    } else {
        Box::new(DirEntry::new(path))
    }
}

impl<'a> DirEntry<'a> {
    fn new(path: &'a str) -> DirEntry<'a> {
        Self { abs_dir: &Path::new(path) }
    }
}

impl<'a> ZipEntry<'a> {
    fn new(path: &'a str) -> ZipEntry<'a> {
        Self { abs_path: &Path::new(path) }
    }
}

impl<'a> CompositeEntry<'a> {
    fn new(path: &'a str) -> CompositeEntry<'a> {
        let mut path_vec = Vec::new();
        let pl: Vec<&'a str> = path.split(if cfg!(windows) { ';' } else { ':' }).collect();
        for p in pl {
            path_vec.push(new_entry(p));
        }
        Self { entrys: path_vec }
    }
}

impl<'a> WildcardEntry<'a> {
    fn new(path: &'a str) -> WildcardEntry<'a> {
        Self { entry: CompositeEntry::new(path) }
    }
}

impl<'a> Entry for DirEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(Vec<u8>, &Entry)> {
        let pb = self.abs_dir.join(className);
        let path = pb.as_path();
        if path.is_file() {
            let file = file_util::path_to_file(path);
            Some((file_util::read_file(&file), self))
        } else {
            None
        }
    }
}

impl<'a> Entry for ZipEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(Vec<u8>, &Entry)> {
        let file = file_util::path_to_file(self.abs_path);
        let classFileName = file_util::classname_to_filename(className);
        let reader = BufReader::new(file);
        let mut za = zip::ZipArchive::new(reader).unwrap();

        let mut file = za.by_name(classFileName).unwrap();
        let mut v = Vec::new();
        file.read_to_end(&mut v);
        Some((v, self))
    }
}

impl<'a> Entry for CompositeEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(Vec<u8>, &Entry)> {
        for entry in &self.entrys {
            let res = entry.read_class(className);
            if res.is_some() {
                return res;
            }
        }
        return None;
    }
}

impl<'a> Entry for WildcardEntry<'a> {
    fn read_class(&self, className: &str) -> Option<(Vec<u8>, &Entry)> {
        self.entry.read_class(className)
    }
}

impl<'a> ToString for DirEntry<'a> {
    fn to_string(&self) -> String {
        self.abs_dir.to_string_lossy().as_ref().to_string()
    }
}

impl<'a> ToString for ZipEntry<'a> {
    fn to_string(&self) -> String {
        self.abs_path.to_string_lossy().as_ref().to_string()
    }
}

impl<'a> ToString for CompositeEntry<'a> {
    fn to_string(&self) -> String {
        let mut strs = Vec::new();
        for entry in &self.entrys {
            strs.push(entry.to_string());
        }
        strs.join(if cfg!(windows) { ";" } else { ":" })
    }
}

impl<'a> ToString for WildcardEntry<'a> {
    fn to_string(&self) -> String {
        self.entry.to_string()
    }
}