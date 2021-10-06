use std::ffi::{CStr, CString};

use libnice_sys::{
    g_inet_address_to_string, g_object_unref, g_resolver_free_addresses, g_resolver_get_default,
    g_resolver_lookup_by_name_finish, g_resolver_lookup_by_name_with_flags, gchar, gpointer,
    strcpy, GAsyncResult, GError, GInetAddress, GObject,
    GResolverNameLookupFlags_G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY,
};

pub fn mdns_resolve(mdns_local_addr: &str) -> Result<String, String> {
    unsafe {
        let resolver = g_resolver_get_default();
        let addr = CString::new(mdns_local_addr);
        let mut error: *mut GError = std::ptr::null_mut();
        let list = g_resolver_lookup_by_name_with_flags(
            resolver,
            addr.unwrap().as_ptr(),
            GResolverNameLookupFlags_G_RESOLVER_NAME_LOOKUP_FLAGS_IPV4_ONLY,
            std::ptr::null_mut(),
            std::ptr::addr_of_mut!(error),
        );
        if !list.is_null() {
            let resolved = g_inet_address_to_string((*list).data as *mut GInetAddress);
            g_resolver_free_addresses(list);
            g_object_unref(resolver as *mut _);
            if !resolved.is_null() {
                return Ok(CStr::from_ptr(resolved).to_str().unwrap().to_owned());
            }
        }

        Err("there was an error resolving mdns address".into())
    }
}

// static void janus_sdp_mdns_resolved(GObject *source_object, GAsyncResult *res, gpointer user_data) {
unsafe extern "C" fn _callback(
    _source_object: *mut GObject,
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
