use super::*;
use std::{ffi::CString, path::Path, ptr::null_mut};

pub struct Mecab(open_jtalk_sys::Mecab);

pub struct MecabFeature;

impl Default for Mecab {
    fn default() -> Self {
        Self(open_jtalk_sys::Mecab {
            feature: null_mut(),
            size: Default::default(),
            model: null_mut(),
            tagger: null_mut(),
            lattice: null_mut(),
        })
    }
}

impl Mecab {
    unsafe fn as_raw_ptr(&mut self) -> *mut open_jtalk_sys::Mecab {
        &mut self.0 as *mut open_jtalk_sys::Mecab
    }
    pub fn initialize(&mut self) -> bool {
        unsafe { bool_number_to_bool(open_jtalk_sys::Mecab_initialize(self.as_raw_ptr())) }
    }

    pub fn load(&mut self, dic_dir: impl AsRef<Path>) -> bool {
        let dic_dir = CString::new(dic_dir.as_ref().to_str().unwrap()).unwrap();
        unsafe {
            bool_number_to_bool(open_jtalk_sys::Mecab_load(
                self.as_raw_ptr(),
                dic_dir.as_ptr(),
            ))
        }
    }

    pub fn get_feature_mut(&mut self) -> &mut MecabFeature {
        unsafe { &mut *(open_jtalk_sys::Mecab_get_feature(self.as_raw_ptr()) as *mut MecabFeature) }
    }

    pub fn analysis(&mut self, str: impl AsRef<str>) -> bool {
        let str = CString::new(str.as_ref()).unwrap();
        unsafe {
            bool_number_to_bool(open_jtalk_sys::Mecab_analysis(
                self.as_raw_ptr(),
                str.as_ptr(),
            ))
        }
    }

    pub fn print(&mut self) -> bool {
        unsafe { bool_number_to_bool(open_jtalk_sys::Mecab_print(self.as_raw_ptr())) }
    }

    pub fn get_size(&mut self) -> i32 {
        unsafe { open_jtalk_sys::Mecab_get_size(self.as_raw_ptr()) }
    }

    pub fn clear(&mut self) -> bool {
        unsafe { bool_number_to_bool(open_jtalk_sys::Mecab_clear(self.as_raw_ptr())) }
    }
}

impl MecabFeature {
    pub(crate) unsafe fn as_raw_ptr(&mut self) -> *mut *mut i8 {
        std::mem::transmute(self)
    }
}