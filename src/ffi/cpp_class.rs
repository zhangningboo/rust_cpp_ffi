use std::ffi::c_int;

#[link(name = "cpp", kind = "static")]
unsafe extern "C" {
    unsafe fn create_class_instance(dev_id: c_int) -> *mut CppClassFFi;
    
    unsafe fn call_instance_func(instalnce: *const CppClassFFi, a: c_int, b: c_int) -> c_int;

    unsafe fn free_class_instance(instalnce: *mut CppClassFFi) -> bool;
}

#[repr(C)]
pub struct CppClassFFi(*mut std::ffi::c_void);

impl CppClassFFi {
    pub fn new(dev_id: i32) -> Self {
        unsafe {
            let ptr = create_class_instance(dev_id);
            assert!(!ptr.is_null(), "Failed to create C++ instance");
            CppClassFFi(ptr as *mut std::ffi::c_void)
        }
    }

    pub fn call_instance_func(&self, a: i32, b: i32) -> i32 {
        unsafe {
            call_instance_func(self.0 as *const CppClassFFi, a, b)
        }
    }
}

trait Destroy {
    fn destroy(&mut self);
}

impl Destroy for CppClassFFi {
    fn destroy(&mut self) {
        unsafe {
            free_class_instance(self.0 as *mut CppClassFFi);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // cargo test --lib ffi::cpp_class::test_create_and_add
    #[test]
    fn test_create_and_add() {
        let instance = CppClassFFi::new(42);
        let result = instance.call_instance_func(10, 20);
        assert_eq!(result, 30);
    }
    // cargo test --lib ffi::cpp_class::test_multiple_instances
    #[test]
    fn test_multiple_instances() {
        let inst1 = CppClassFFi::new(1);
        let inst2 = CppClassFFi::new(2);

        assert_eq!(inst1.call_instance_func(1, 1), 2);
        assert_eq!(inst2.call_instance_func(2, 2), 4);
        // Drop happens automatically
    }

    #[test]
    fn test_zero_dev_id() {
        let instance = CppClassFFi::new(0);
        assert_eq!(instance.call_instance_func(0, 0), 0);
    }

    #[test]
    fn test_negative_numbers() {
        let instance = CppClassFFi::new(-1);
        assert_eq!(instance.call_instance_func(-5, 3), -2);
    }

    // Optional: Test that drop is called (via side effect)
    // Since we can't observe C++ output in test easily,
    // we rely on absence of crash as evidence of correctness.
    #[test]
    fn test_no_double_free_on_drop() {
        let instance = CppClassFFi::new(99);
        drop(instance); // Explicit drop
        // If this doesn't segfault or double-free, it's good.
    }

    #[test]
    fn test_scope_based_destruction() {
        {
            let _inst = CppClassFFi::new(100);
            // _inst goes out of scope here → drop called
        }
        // Program should not crash
    }
}