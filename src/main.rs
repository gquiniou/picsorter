extern crate rexiv2;
extern crate chrono;
use std::env;
use std::path::Path;
use chrono::*;


fn processfolder(path: &Path) {
    match path.read_dir() {
        Err(why) => println!("Could not access folder '{:?}':  {:?}", path, why.kind()),
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    processpic(entry.path().as_path());

                    // if let Ok(filetype) = entry.file_type() {
                    //     if filetype.is_file() {
                    //         processpic(entry.path().as_path());
                    //     }
                    // }
                    
                }  else {
                    println!("could not get entry {:?}", entry);
                }
            }
            // for file in paths.filter( |x| if let Ok(x) = x {if let Ok(t) = x.file_type() { t.is_dir()} else {false}}else {false}) {
            //     let file = file.unwrap();
            //     //processpic(file);
            // }
        }
    }
}

fn processpic(path: &Path) {
    println!("*Processing file {:?}", path);
    if ! path.is_dir() {
        match rexiv2::Metadata::new_from_path(path) {
            Ok(meta) => {
                if meta.has_tag("Exif.Photo.DateTimeOriginal") {
                    println!("    Exif.Photo.DateTimeOriginal  {:?}", meta.get_tag_string("Exif.Photo.DateTimeOriginal").unwrap());
                    let d = NaiveDateTime::parse_from_str(&meta.get_tag_string("Exif.Photo.DateTimeOriginal").unwrap(), "%Y:%m:%d %H:%M:%S").unwrap();
                    println!("d: {:?}", d);
                }
            },
            Err(err) => println!("{:?}", err)
        }
    }
}

fn gettargetfolder() {
}

fn movepic() {
}

fn getexifdate() {
}

fn main() {
   let myargs: Vec<String> = env::args().collect();

   if myargs.len() == 1 {
       println!("usage: picsorter <directory>");
   } else {

       for myarg in myargs.iter().skip(1) {

         let mypath = Path::new(myarg);
	 if !mypath.is_dir() {
	    println!("'{:?}' is not a directory", mypath);
	 } else {
             processfolder(mypath);
         } 
     }
   }   
}
