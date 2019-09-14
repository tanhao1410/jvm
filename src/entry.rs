use crate::util::file_util;
use std::path::Path;
use std::option::Option;
use std::string::ToString;
use std::io::{BufReader, Read};
use std::error::Error;

pub trait Entry: ToString {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, &dyn Entry)>;
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

pub fn new_entry<'a>(path: &'a str) -> Box<dyn Entry + 'a> {
    if path.contains(if cfg!(windows) { ';' } else { ':' }) {
        Box::new(CompositeEntry::new(path))
    } else if path.ends_with("*") {
        Box::new(WildcardEntry::new(path))
    } else if file_util::is_jar_name(path) {
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
        let paths: Vec<&'a str> = path.split(if cfg!(windows) { ';' } else { ':' }).collect();
        Self::new_by_paths(paths)
    }
    fn new_by_paths(paths: Vec<&'a str>) -> CompositeEntry<'a> {
        let mut entrys = Vec::new();
        for p in paths {
            entrys.push(new_entry(p));
        }
        Self::new_by_entrys(entrys)
    }
    fn new_by_entrys(entrys: Vec<Box<dyn Entry + 'a>>) -> CompositeEntry<'a> {
        Self { entrys: entrys }
    }
}

impl<'a> WildcardEntry<'a> {
    fn new(path: &'a str) -> WildcardEntry<'a> {
        let mut p = path.to_string();
        p.remove(path.len() - 1);
        let read_dir = std::fs::read_dir(p).unwrap();

        let mut paths: Vec<&'a str> = Vec::new();

        for result_dir_entry in read_dir {
            let p = result_dir_entry.unwrap().path();
            let path_in:&'a Path = p.as_path();

            if path_in.is_file() {
                let file_name:&'a str = path_in.file_name().unwrap().to_str().unwrap();

                if file_util::is_jar_name(file_name) {
                    let s:&'a str = path_in.to_str().unwrap();
                    paths.push(&s);
                }
            }
        }
        Self { entry: CompositeEntry::new_by_paths(paths) }
    }
}

impl<'a> Entry for DirEntry<'a> {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, &dyn Entry)> {
        let pb = self.abs_dir.join(class_name);
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
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, &dyn Entry)> {
        let file = file_util::path_to_file(self.abs_path);
        let reader = BufReader::new(file);
        let mut za = zip::ZipArchive::new(reader).unwrap();

        // println!("aaaaaaaaaaaaaaaaaaaaaaaaa {}", class_name);

        let mut file = za.by_name(class_name).unwrap();
        let mut v = Vec::new();
        let read_res = file.read_to_end(&mut v);
        if read_res.is_err() {
            panic!("ZipEntry read file err {}", read_res.unwrap_err().description());
        }
        Some((v, self))
    }
}

impl<'a> Entry for CompositeEntry<'a> {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, &dyn Entry)> {
        for entry in &self.entrys {
            let res = entry.read_class(class_name);
            if res.is_some() {
                return res;
            }
        }
        return None;
    }
}

impl Entry for WildcardEntry<'_> {
    fn read_class(&self, class_name: &str) -> Option<(Vec<u8>, &dyn Entry)> {
        self.entry.read_class(class_name)
    }
}

impl ToString for DirEntry<'_> {
    fn to_string(&self) -> String {
        self.abs_dir.to_string_lossy().as_ref().to_string()
    }
}

impl ToString for ZipEntry<'_> {
    fn to_string(&self) -> String {
        self.abs_path.to_string_lossy().as_ref().to_string()
    }
}

impl ToString for CompositeEntry<'_> {
    fn to_string(&self) -> String {
        let mut strs = Vec::new();
        for entry in &self.entrys {
            strs.push(entry.to_string());
        }
        strs.join(if cfg!(windows) { ";" } else { ":" })
    }
}

impl ToString for WildcardEntry<'_> {
    fn to_string(&self) -> String {
        self.entry.to_string()
    }
}