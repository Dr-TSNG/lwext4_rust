use crate::bindings::*;
use alloc::ffi::CString;
use core::mem::MaybeUninit;

pub struct Ext4File(ext4_file);

impl Ext4File {
    /// File open function.
    ///
    /// |---------------------------------------------------------------|
    /// |   r or rb                 O_RDONLY                            |
    /// |---------------------------------------------------------------|
    /// |   w or wb                 O_WRONLY|O_CREAT|O_TRUNC            |
    /// |---------------------------------------------------------------|
    /// |   a or ab                 O_WRONLY|O_CREAT|O_APPEND           |
    /// |---------------------------------------------------------------|
    /// |   r+ or rb+ or r+b        O_RDWR                              |
    /// |---------------------------------------------------------------|
    /// |   w+ or wb+ or w+b        O_RDWR|O_CREAT|O_TRUNC              |
    /// |---------------------------------------------------------------|
    /// |   a+ or ab+ or a+b        O_RDWR|O_CREAT|O_APPEND             |
    /// |---------------------------------------------------------------|
    pub fn open(path: &str, flags: u32) -> Result<Self, i32> {
        let c_path = CString::new(path).expect("CString::new failed");
        unsafe {
            let mut file = MaybeUninit::uninit();
            match ext4_fopen2(file.as_mut_ptr(), c_path.as_ptr(), flags as i32) {
                0 => Ok(Self(file.assume_init())),
                r => {
                    error!("ext4_fopen {} failed: {}", path, r);
                    Err(r)
                }
            }
        }
    }

    pub fn seek(&mut self, offset: i64, seek_type: u32) -> Result<(), i32> {
        let mut offset = offset;
        let size = self.size() as i64;
        if offset > size {
            warn!("Seek beyond the end of the file");
            offset = size;
        }
        match unsafe { ext4_fseek(&mut self.0, offset, seek_type) } {
            0 => Ok(()),
            e => {
                error!("ext4_fseek failed: {}", e);
                Err(e)
            }
        }
    }

    pub fn read(&mut self, buff: &mut [u8]) -> Result<usize, i32> {
        let mut rw_count = 0;
        unsafe {
            match ext4_fread(&mut self.0, buff.as_mut_ptr() as _, buff.len(), &mut rw_count) {
                0 => Ok(rw_count),
                e => {
                    error!("ext4_fread failed: {}", e);
                    Err(e)
                }
            }
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, i32> {
        let mut rw_count = 0;
        unsafe {
            match ext4_fwrite(&mut self.0, buf.as_ptr() as _, buf.len(), &mut rw_count) {
                0 => Ok(rw_count),
                e => {
                    error!("ext4_fwrite failed: {}", e);
                    Err(e)
                }
            }
        }
    }

    pub fn truncate(&mut self, size: u64) -> Result<usize, i32> {
        debug!("file_truncate to {}", size);
        let r = unsafe { ext4_ftruncate(&mut self.0, size) };
        if r != EOK as i32 {
            error!("ext4_ftruncate: rc = {}", r);
            return Err(r);
        }
        Ok(EOK as usize)
    }

    pub fn size(&mut self) -> u64 {
        unsafe {
            ext4_fsize(&mut self.0)
        }
    }
}

impl Drop for Ext4File {
    fn drop(&mut self) {
        unsafe {
            ext4_fclose(&mut self.0);
        }
    }
}
