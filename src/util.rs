
use serde_json as json;
//use serde::{Serialize, Deserialize};

use std::io;
use std::fs;
use std::path::PathBuf;
//use std::time::{SystemTime, Duration, UNIX_EPOCH};

fn sort_dir_entries(dir: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut sortee: Vec<(std::time::SystemTime, PathBuf)> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        match entry.path().extension() {
            None => continue,
            Some(ext) => {
                // the extension's being .metadata is not related to me asking
                // for the file's metadata() here.
                if ext=="metadata" {
                    let md = path.metadata()?;
                    let time = md.modified()?;
                    sortee.push((time, path));
                };
            },
        };
    };

    sortee.sort_unstable_by_key(|e| e.0);
    sortee.reverse();

    Ok(sortee.iter().map(|p| p.1.clone()).collect())
}

pub fn last_modified_notebook(root_dir: &PathBuf) -> io::Result<PathBuf> {
    
    let mut last = (0, PathBuf::new());

    if root_dir.is_dir() {
        let mut sorted = sort_dir_entries(root_dir)?;
        sorted.truncate(10);
        for entry in sorted {
            match entry.extension() {
                None => continue,
                Some(_ext) => {
                    // _ext == "metadata"
                    let j: json::Value = json::from_reader(fs::File::open(&entry)?)?;

                    if let json::Value::String(ts) = &j["lastModified"] {
                        let this = &ts[..13];
                        if let Ok(this) = this.parse::<u64>() {
                            if last.0 > this {
                                continue
                            }
                            else {
                                last = (this,entry.clone());
                            };
                        };
                    };
                },
            };
        };
    }
    else {
        dbg!(root_dir);
    };

    Ok(last.1)
}


pub fn last_modified_page(root_dir: &PathBuf) -> io::Result<PathBuf> {

    let last_notebook = last_modified_notebook(root_dir)?;
    dbg!(&last_notebook);
    let notebook_uuid = last_notebook.file_stem().unwrap();
    let mut content_file = PathBuf::new();
    content_file.push(root_dir);
    content_file.push(notebook_uuid);

    let mut out = content_file.clone();

    content_file.set_extension("content");

    let meta: json::Value = json::from_reader(fs::File::open(&last_notebook)?)?;

    let content: json::Value = json::from_reader(fs::File::open(&content_file)?)?;

    if let json::Value::Number(pnum) = &meta["lastOpenedPage"] {
        if let json::Value::Object(cpages) = &content["cPages"] {
            if let json::Value::Object(page) = &cpages["lastOpened"] {
                if let json::Value::String(value) = &page["value"] {
                out.push(value);
                out.set_extension("rm");
                };
            }
            else {
                dbg!("str");
            };
        }
        else {
            if let json::Value::Array(pages) = &content["pages"] {
                if let json::Value::String(page) = &pages[pnum.as_u64().unwrap() as usize] {
                    out.push(page);
                    out.set_extension("rm");
                };
            }
            else {
                dbg!("arr");
            };
        };
    }
    else {
        dbg!("num");
    };
    
    dbg!(&out);
    Ok(out)
}

