use std::ffi::CString;

use libnice_sys::{
    g_resolver_get_default, g_resolver_lookup_by_name_async, gpointer, GAsyncResult, GObject,
};

pub fn mdns_resolver(addr: &str) -> Result<String, String> {
    unsafe {
        let resolver = g_resolver_get_default();
        g_resolver_lookup_by_name_async(
            resolver,
            CString::new(addr).unwrap().as_ptr(),
            std::ptr::null_mut(),
            Some(callback),
            std::ptr::null_mut(),
        );
    }
    Err("there was an error resolving mdns address".into())
}

// static void janus_sdp_mdns_resolved(GObject *source_object, GAsyncResult *res, gpointer user_data) {
unsafe extern "C" fn callback(
    source_object: *mut GObject,
    res: *mut GAsyncResult,
    user_data: gpointer,
) {
}
