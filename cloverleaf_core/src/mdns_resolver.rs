use std::ffi::CString;

use libc::c_char;
use libnice_sys::{
    g_inet_address_to_string, g_resolver_get_default, g_resolver_lookup_by_name_async,
    g_resolver_lookup_by_name_finish, gchar, gpointer, strcpy, GAsyncResult, GError, GInetAddress,
    GObject,
};

pub fn mdns_resolver(addr: &str) -> Result<String, String> {
    unsafe {
        let resolver = g_resolver_get_default();
        let result: *mut c_char = std::ptr::null_mut();
        let addr = CString::new(addr);
        g_resolver_lookup_by_name_async(
            resolver,
            addr.unwrap().as_ptr(),
            std::ptr::null_mut(),
            Some(callback),
            result as gpointer,
        );
        if result.is_null() {
            println!("result is null");
            Err("there was an error resolving mdns address".into())
        } else {
            Ok(CString::from_raw(result as *mut _).into_string().unwrap())
        }
    }
}

// static void janus_sdp_mdns_resolved(GObject *source_object, GAsyncResult *res, gpointer user_data) {
unsafe extern "C" fn callback(
    source_object: *mut GObject,
    res: *mut GAsyncResult,
    user_data: gpointer,
) {
    let resolver = g_resolver_get_default();
    let mut error: *mut GError = std::ptr::null_mut();
    let list = g_resolver_lookup_by_name_finish(resolver, res, std::ptr::addr_of_mut!(error));
    let resolved = g_inet_address_to_string((*list).data as *mut GInetAddress);
    let mut user_data: *mut gchar = user_data as *mut _;
    user_data = libnice_sys::g_malloc0(libnice_sys::strlen(resolved) + 1) as *mut _;
    strcpy(user_data as *mut _, resolved);
}
