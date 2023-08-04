
use serde_json as json;
//use serde::{Serialize, Deserialize};

use std::io;
use std::fs;
use std::path::{Path,PathBuf};
//use std::time::{SystemTime, Duration, UNIX_EPOCH};


pub fn last_modified_notebook(root_dir: &Path) -> io::Result<PathBuf> {
    
    let mut last = (0, PathBuf::new());

    if root_dir.is_dir() {
        for entry in fs::read_dir(root_dir)? {
            let entry = entry?;
            let path = entry.path();
            match entry.path().extension() {
                None => continue,
                Some(ext) => {
                    if ext == "metadata" {
                        let j: json::Value = json::from_reader(fs::File::open(&path)?)?;

                        if let json::Value::String(ts) = &j["lastModified"] {
                            let this = &ts[..13];
                            if let Ok(this) = this.parse::<u64>() {
                                if last.0 > this {
                                    continue
                                }
                                else {
                                    last = (this,path.clone());
                                };
                            };
                        };
                    };
                },
            };
        };
    };

    Ok(last.1)
}


pub fn last_modified_page(root_dir: &Path) -> io::Result<PathBuf> {

    let last_notebook = last_modified_notebook(root_dir)?;
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
    
    Ok(out)
}

