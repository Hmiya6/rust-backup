use std::path::Path;
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::fs;
use zip::{ZipWriter, write::FileOptions, CompressionMethod};
use chrono::Utc;
use std::io::Write;

pub struct Backup {
    src: String,
    dst: String,
    old_digest: String,
}

impl Backup {

    pub fn new(src: &str, dst: &str) -> Result<Self> {
        
        // validate src and dst
        let src_path = Path::new(src);
        let dst_path = Path::new(dst);
        if !src_path.exists() {
            return Err(anyhow!("Invalid source path: {}", src));
        }
        if !dst_path.exists() {
            return Err(anyhow!("Invalid destination path: {}", dst));
        }
        
        // return 
        Ok(Self {
            src: String::from(src),
            dst: String::from(dst),
            old_digest: String::from(""),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        if self.diff()? {
            self.archive()?;
        }
        Ok(())
    }
    
    // diff src hash and old_digest hash
    fn diff(&mut self) -> Result<bool> {
        
        // directory hasher
        let mut outer_hasher = Sha256::new();
        
        // file walk
        // recursive fs::read_dir() can be alternative
        for entry in WalkDir::new(&self.src) {
            let file = entry?;
            
            // skip if the file is directory
            if file.file_type().is_dir() {
                continue;
            }
            
            // get hash value of the file
            let path = file.path();
            let file_hash = Sha256::new()
                .chain(fs::read(path)?)
                .chain(format!("{}", path.display()))
                .finalize();
            
            // append the hash value to dir hasher
            outer_hasher.update(format!("{:x}", file_hash));
        }
        
        // get directory hash
        let dir_hash = outer_hasher.finalize();
        let dir_digest = format!("{:x}", dir_hash);
        
        // return flag
        let mut flag = true;
        if self.old_digest == dir_digest {
            flag = false;
        } else {
            flag = true;
            self.old_digest = dir_digest;
        }
        Ok(flag)
    }


    fn archive(&self) -> Result<()> {
        
        // set file name
        let now = Utc::now();
        let file_name = format!("{}/backup_{}.zip", self.dst, now);
        
        // create zip file
        let zip_file = fs::File::create(Path::new(&file_name))?;
        
        // write directory into the zip file
        let mut writer = ZipWriter::new(zip_file);
        let options = FileOptions::default().compression_method(CompressionMethod::Stored);
        // dirwalk
        for entry in WalkDir::new(&self.src) {
            if let Ok(file) = entry {
                // relative path from src
                let path = format!("{}", file.path().display());
                if file.file_type().is_dir() {
                    // add directory into the zip file
                    writer.add_directory(&path, options)?;
                } else {
                    // add file into the zip file
                    writer.start_file(&path, options)?;
                    writer.write_all(&fs::read(file.path())?)?;
                }
            } 
        }
        writer.finish()?;

        // return 
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_backup_new_success() {
        let backup = Backup::new("..", "..").unwrap();
        assert_eq!(backup.src, String::from(".."));
        assert_eq!(backup.dst, String::from(".."));
    }

    #[test]
    fn test_backup_new_failure() {
        let mut flag = false;
        if let Err(_) = Backup::new("¥€$", "piyopiyopiyo") {
            flag = true;
        }
        assert!(flag);
    }
    #[test]
    fn test_diff() {
        let mut backup = Backup::new("./src", ".").unwrap();
        let flag = backup.diff().unwrap();
        // println!("{}", backup.old_digest);
        assert!(flag);
    }
    
    #[test]
    fn tset_archive() {
        let backup = Backup::new("./src", ".").unwrap();
        backup.archive().unwrap();
    }
}
