use alloc::ffi::CString;
use alloc::string::{String, ToString};
use core::mem::MaybeUninit;
use crate::bindings::*;

pub struct Ext4Dir(ext4_dir);

pub struct Ext4DirEntry {
    pub inode: u32,
    pub name: String,
    pub type_: u8,
}

impl Ext4Dir {
    pub fn open(path: &str) -> Result<Self, i32> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let mut dir = MaybeUninit::uninit();
            match ext4_dir_open(dir.as_mut_ptr(), c_path.as_ptr()) {
                0 => Ok(Ext4Dir(dir.assume_init())),
                e => {
                    error!("ext4_dir_open {} failed: {}", path, e);
                    Err(e)
                }
            }
        }
    }

    pub fn inode(&self) -> u32 {
        self.0.f.inode
    }

    pub fn mkdir(path: &str) -> Result<(), i32> {
        let c_path = CString::new(path).unwrap();
        match unsafe { ext4_dir_mk(c_path.as_ptr()) } {
            0 => Ok(()),
            e => {
                error!("ext4_dir_mk {} failed: {}", path, e);
                Err(e)
            }
        }
    }

    pub fn rmdir(path: &str) -> Result<(), i32> {
        let c_path = CString::new(path).unwrap();
        match unsafe { ext4_dir_rm(c_path.as_ptr()) } {
            0 => Ok(()),
            e => {
                error!("ext4_dir_rm {} failed: {}", path, e);
                Err(e)
            }
        }
    }

    pub fn rmfile(path: &str) -> Result<(), i32> {
        let c_path = CString::new(path).unwrap();
        match unsafe { ext4_fremove(c_path.as_ptr()) } {
            0 => Ok(()),
            e => {
                error!("ext4_dir_rm {} failed: {}", path, e);
                Err(e)
            }
        }
    }

    pub fn movedir(old_path: &str, new_path: &str) -> Result<(), i32> {
        let c_old_path = CString::new(old_path).unwrap();
        let c_new_path = CString::new(new_path).unwrap();
        match unsafe { ext4_dir_mv(c_old_path.as_ptr(), c_new_path.as_ptr()) } {
            0 => Ok(()),
            e => {
                error!("ext4_dir_mv {} to {} failed: {}", old_path, new_path, e);
                Err(e)
            }
        }
    }

    pub fn movefile(old_path: &str, new_path: &str) -> Result<(), i32> {
        let c_old_path = CString::new(old_path).unwrap();
        let c_new_path = CString::new(new_path).unwrap();
        match unsafe { ext4_frename(c_old_path.as_ptr(), c_new_path.as_ptr()) } {
            0 => Ok(()),
            e => {
                error!("ext4_dir_mv {} to {} failed: {}", old_path, new_path, e);
                Err(e)
            }
        }
    }

    pub fn readlink(path: &str) -> Result<String, i32> {
        let c_path = CString::new(path).unwrap();
        let mut buf = [0u8; 260];
        let mut rcnt = 0;
        unsafe {
            match ext4_readlink(c_path.as_ptr(), buf.as_mut_ptr() as _, 260, &mut rcnt) {
                0 => Ok(String::from_utf8_lossy(&buf[..rcnt as usize]).to_string()),
                e => {
                    error!("ext4_readlink {} failed: {}", path, e);
                    Err(e)
                }
            }
        }
    }
}

impl Drop for Ext4Dir {
    fn drop(&mut self) {
        unsafe {
            ext4_dir_close(&mut self.0);
        }
    }
}

impl Iterator for Ext4Dir {
    type Item = Ext4DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let entry = unsafe {
            ext4_dir_entry_next(&mut self.0).as_ref()?
        };
        let name_buf = &entry.name[..entry.name_length as usize];
        Some(Ext4DirEntry {
            inode: entry.inode,
            name: String::from_utf8_lossy(name_buf).to_string(),
            type_: entry.inode_type,
        })
    }
}
