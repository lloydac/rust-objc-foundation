use {NSString, INSString, INSObject, NSDictionary, INSDictionary};
use objc_id::Id;

#[allow(improper_ctypes)]
#[link(name = "Foundation", kind = "framework")]
extern {
    // User info dictionary keys
    static NSLocalizedDescriptionKey: &'static NSString;

    // Error domains
    static NSCocoaErrorDomain: &'static NSString;
    static NSPOSIXErrorDomain: &'static NSString;
    static NSOSStatusErrorDomain: &'static NSString;
    static NSMachErrorDomain: &'static NSString;
}

object_struct!(NSError);

impl NSError {

    /// Create a new NSError with the given error domain, error code, and description.
    pub fn new(domain: &str, code: isize, localized_description: &str) -> Id<NSError> {
        unsafe {
            let domain = NSString::from_str(domain);
            let desc = NSString::from_str(localized_description);
            let user_info = NSDictionary::from_keys_and_objects(&[NSLocalizedDescriptionKey], vec![desc]);

            let class = Self::class();
            let mut err: *mut NSError = msg_send![class, alloc];
            err = msg_send![err, initWithDomain:&*domain code:code userInfo:&*user_info];

            Id::from_retained_ptr(err)
        }
    }

    /// Get the error code.
    pub fn code(&self) -> isize {
        unsafe {
            msg_send![self, code]
        }
    }

    /// Get the error domain.
    pub fn domain(&self) -> &str {
        unsafe {
            let domain: *mut NSString = msg_send![self, domain];
            assert!(!domain.is_null());
            (&*domain).as_str()
        }
    }

    fn has_domain(&self, expected_domain: &NSString) -> bool {
        unsafe {
            let domain: *mut NSString = msg_send![self, domain];
            assert!(!domain.is_null());
            &*domain == expected_domain
        }
    }

    /// True if the error domain is NSCocoaErrorDomain
    pub fn is_cocoa_error(&self) -> bool {
        unsafe { self.has_domain(NSCocoaErrorDomain) }
    }

    /// True if the error domain is NSPOSIXErrorDomain
    pub fn is_posix_error(&self) -> bool {
        unsafe { self.has_domain(NSPOSIXErrorDomain) }
    }

    /// True if the error domain is NSOSStatusErrorDomain
    pub fn is_os_status_error(&self) -> bool {
        unsafe { self.has_domain(NSOSStatusErrorDomain) }
    }

    /// True if the error domain is NSMachErrorDomain
    pub fn is_mach_error(&self) -> bool {
        unsafe { self.has_domain(NSMachErrorDomain) }
    }

    /// Get the localized description of this error.
    pub fn localized_description(&self) -> &str {
        unsafe {
            let desc: *mut NSString = msg_send![self, localizedDescription];
            assert!(!desc.is_null());
            (&*desc).as_str()
        }
    }
}

#[cfg(test)]
mod tests {
    use NSError;

    #[test]
    fn test_error_properties() {
        let domain = "MyDomain";
        let code = 42;
        let desc = "this is the description";
        let err = NSError::new(domain, code, desc);

        assert_eq!(err.domain(), domain);
        assert_eq!(err.code(), code);
        assert_eq!(err.localized_description(), desc);
    }

    #[test]
    fn test_cocoa_domain() {
        let err = NSError::new("NSCocoaErrorDomain", 42, "description");
        assert!(err.is_cocoa_error());
        assert!(!err.is_posix_error());
        assert!(!err.is_os_status_error());
        assert!(!err.is_mach_error());
    }

    #[test]
    fn test_posix_domain() {
        let err = NSError::new("NSPOSIXErrorDomain", 42, "description");
        assert!(!err.is_cocoa_error());
        assert!(err.is_posix_error());
        assert!(!err.is_os_status_error());
        assert!(!err.is_mach_error());
    }

    #[test]
    fn test_os_status_domain() {
        let err = NSError::new("NSOSStatusErrorDomain", 42, "description");
        assert!(!err.is_cocoa_error());
        assert!(!err.is_posix_error());
        assert!(err.is_os_status_error());
        assert!(!err.is_mach_error());
    }

    #[test]
    fn test_mach_domain() {
        let err = NSError::new("NSMachErrorDomain", 42, "description");
        assert!(!err.is_cocoa_error());
        assert!(!err.is_posix_error());
        assert!(!err.is_os_status_error());
        assert!(err.is_mach_error());
    }
}
