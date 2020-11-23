extern crate rexiv2;
extern crate chrono;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use chrono::*;
use rexiv2::Rexiv2Error::*;
use std::fs;

fn processfolder(path: &Path) {
    match path.read_dir() {
        Err(why) => eprintln!("Could not access folder '{:?}':  {:?}", path, why.kind()),
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    processpic(entry.path().as_path());
                }  else {
                    eprintln!("could not get entry {:?}", entry);
                }
            }
        }
    }
}

fn processpic(path: &Path) {
    if ! path.is_dir() {
        println!("*Processing file {:?}", path);
        match rexiv2::Metadata::new_from_path(path) {
            Ok(meta) => {
                if meta.has_tag("Exif.Photo.DateTimeOriginal") {
                    println!("    Exif.Photo.DateTimeOriginal  {:?}", meta.get_tag_string("Exif.Photo.DateTimeOriginal").unwrap());
                    let d = NaiveDateTime::parse_from_str(&meta.get_tag_string("Exif.Photo.DateTimeOriginal").unwrap(), "%Y:%m:%d %H:%M:%S").unwrap();
                    let mut f = get_target_folder(d, path.parent().unwrap());
                    f.push(path.file_name().unwrap());
                    println!("Moving '{:?}' to '{:?}'", path, f);
                    if let Err(why) = fs::rename(path, f) { 
                        eprintln!("Could not move file: {:?}", why.kind()) 
                    }
                } else {
                    println!("no Exif.Photo.DateTimeOriginal");
                }
            },
            Err(err) => match err {
                NoValue => eprintln!("    Could not get metadata for {:?}: unspecified error", path),
                Utf8(utf8err) => eprintln!("    Could not get metadata for {:?}: {:?}", path, utf8err),
                Internal(interr) => eprintln!("    Could not get metadata for {:?}: {:?}", path, interr.unwrap_or_else(|| String::from("unspecified"))),
            }
        }
    }
}

fn get_target_folder(date: NaiveDateTime, parent: &Path) -> PathBuf {
    let dirname = format!("{} {:02}", date.year(), date.month());
    let newpath = parent.join(dirname);
    if !newpath.exists() {
        if let Err(why) = fs::create_dir(&newpath) { 
            eprintln!("Could not create dir ! {:?}", why.kind()) 
        }         
    }
    newpath
}

fn main() {
   let myargs: Vec<String> = env::args().collect();

   if myargs.len() == 1 {
       eprintln!("usage: picsorter <directory>");
    } else {
        for myarg in myargs.iter().skip(1) {
            let mypath = Path::new(myarg);
            if !mypath.is_dir() {
                eprintln!("'{:?}' is not a directory", mypath);
            } else {
                processfolder(mypath);
            } 
        }
   }   
}
