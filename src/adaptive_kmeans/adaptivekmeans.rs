//Copyright (c) 2024 Arithoptix Pty Ltd.
// adaptivekmeans module
use libc::{c_double, c_int, c_void};

 #[repr(C)]
pub struct Point {
    pub coords: *const c_double,
    pub len: usize,
}

impl Point {
    pub fn from_vec(coords: Vec<c_double>) -> Self {
        let len = coords.len();
        let coords_ptr = coords.as_ptr();
        //We must prevent Rust from freeing `coords` when it goes out of scope
        std::mem::forget(coords);
        Point {
            coords: coords_ptr,
            len,
        }
    }
}

impl Drop for Point {
    fn drop(&mut self) {
        // Safety: Convert pointer back to Vec and drop it properly
        unsafe {
            if !self.coords.is_null() {
                Vec::<c_double>::from_raw_parts(self.coords as *mut c_double, self.len, self.len);
            }
        }
    }
}

#[repr(C)]
pub struct KMeansResult {
    pub assignments: *const c_int,
    pub assignments_len: usize,
    pub error_message: *const c_void, //NB. Use *const c_void for Option<String> to handle null pointers
}

impl KMeansResult {
    pub fn new(assignments: Vec<c_int>, error_message: Option<String>) -> Self {
        let assignments_len = assignments.len();
        let assignments_ptr = assignments.as_ptr();
        std::mem::forget(assignments); //We must prevent Rust from trying to free `assignments` when it goes out of scope
        let error_message_ptr = match error_message {
            Some(msg) => {
                let msg = Box::new(msg.into_bytes());  //NB. Now Convert String to Vec<u8> and box it
                Box::into_raw(msg) as *const c_void
            }
            None => std::ptr::null(),
        };
        KMeansResult {
            assignments: assignments_ptr,
            assignments_len,
            error_message: error_message_ptr,
        }
    }
}

impl Drop for KMeansResult {
    fn drop(&mut self) {
        unsafe {
            if !self.assignments.is_null() {
                Vec::from_raw_parts(self.assignments as *mut c_int, self.assignments_len, self.assignments_len);
            }
            if !self.error_message.is_null() {
                //Box::from_raw(self.error_message as *mut Vec<u8>);
                let _ = Box::from_raw(self.error_message as *mut Vec<u8>);
            }
        }
    }
}

extern "C" {
    pub fn adaptive_kmeans(data: *const Point, len: usize, k: usize, max_iterations: usize, use_mahalanobis: bool, result: *mut KMeansResult);
}

pub fn run_adaptive_kmeans(data: Vec<Vec<c_double>>, k: usize, max_iterations: usize, use_mahalanobis: bool) -> KMeansResult {
    let data = data.into_iter().map(Point::from_vec).collect::<Vec<_>>();
    let mut result = KMeansResult::new(vec![0; data.len()], None);

    unsafe {
        adaptive_kmeans(data.as_ptr(), data.len(), k, max_iterations, use_mahalanobis, &mut result);
    }
    result
}
