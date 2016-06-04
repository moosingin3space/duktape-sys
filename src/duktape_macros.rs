use std::os::raw::{c_char, c_void};
use std::ptr;
use duktape;
use duktape_consts::*;

#[inline(always)]
pub unsafe fn duk_create_heap_default() -> *mut duktape::duk_context {
    duktape::duk_create_heap(None, None, None, ptr::null_mut(), None)
}

#[inline(always)]
pub unsafe fn duk_xmove_top(to_ctx: *mut duktape::duk_context,
                            from_ctx: *mut duktape::duk_context,
                            count: duktape::duk_idx_t) {
    duktape::duk_xcopymove_raw(to_ctx, from_ctx, count, 0);
}

#[inline(always)]
pub unsafe fn duk_xcopy_top(to_ctx: *mut duktape::duk_context,
                            from_ctx: *mut duktape::duk_context,
                            count: duktape::duk_idx_t) {
    duktape::duk_xcopymove_raw(to_ctx, from_ctx, count, 1);
}

#[inline(always)]
pub unsafe fn duk_push_string_file(ctx: *mut duktape::duk_context,
                                   path: *const c_char)
                                   -> *const c_char {
    duktape::duk_push_string_file_raw(ctx, path, 0)
}

#[inline(always)]
pub unsafe fn duk_push_thread(ctx: *mut duktape::duk_context) -> duktape::duk_idx_t {
    duktape::duk_push_thread_raw(ctx, 0)
}

#[inline(always)]
pub unsafe fn duk_push_thread_new_globalenv(ctx: *mut duktape::duk_context) -> duktape::duk_idx_t {
    duktape::duk_push_thread_raw(ctx, DUK_THREAD_NEW_GLOBAL_ENV)
}

const DUK_BUF_FLAG_DYNAMIC: duktape::duk_small_uint_t = (1 << 0);
const DUK_BUF_FLAG_EXTERNAL: duktape::duk_small_uint_t = (1 << 1);
// const DUK_BUF_FLAG_NOZERO: duktape::duk_small_uint_t = (1 << 2);

#[inline(always)]
pub unsafe fn duk_push_buffer(ctx: *mut duktape::duk_context,
                              size: duktape::duk_size_t,
                              dynamic: bool)
                              -> *mut c_void {
    duktape::duk_push_buffer_raw(ctx,
                                 size,
                                 if dynamic {
                                     DUK_BUF_FLAG_DYNAMIC
                                 } else {
                                     0
                                 })
}

#[inline(always)]
pub unsafe fn duk_push_fixed_buffer(ctx: *mut duktape::duk_context, size: duktape::duk_size_t) {
    duktape::duk_push_buffer_raw(ctx, size, 0);
}

#[inline(always)]
pub unsafe fn duk_push_dynamic_buffer(ctx: *mut duktape::duk_context,
                                      size: duktape::duk_size_t)
                                      -> *mut c_void {
    duktape::duk_push_buffer_raw(ctx, size, DUK_BUF_FLAG_DYNAMIC)
}

#[inline(always)]
pub unsafe fn duk_push_external_buffer(ctx: *mut duktape::duk_context) {
    duktape::duk_push_buffer_raw(ctx, 0, DUK_BUF_FLAG_DYNAMIC | DUK_BUF_FLAG_EXTERNAL);
}

#[inline(always)]
pub unsafe fn duk_is_callable(ctx: *mut duktape::duk_context,
                              index: duktape::duk_idx_t)
                              -> duktape::duk_bool_t {
    duktape::duk_is_function(ctx, index)
}

#[inline(always)]
pub unsafe fn duk_is_primitive(ctx: *mut duktape::duk_context,
                               index: duktape::duk_idx_t)
                               -> duktape::duk_bool_t {
    duktape::duk_check_type_mask(ctx,
                                 index,
                                 DUK_TYPE_MASK_UNDEFINED | DUK_TYPE_MASK_NULL |
                                 DUK_TYPE_MASK_BOOLEAN |
                                 DUK_TYPE_MASK_NUMBER |
                                 DUK_TYPE_MASK_STRING |
                                 DUK_TYPE_MASK_BUFFER |
                                 DUK_TYPE_MASK_POINTER |
                                 DUK_TYPE_MASK_LIGHTFUNC)
}

#[inline(always)]
pub unsafe fn duk_is_object_coercible(ctx: *mut duktape::duk_context,
                                      index: duktape::duk_idx_t)
                                      -> duktape::duk_bool_t {
    duktape::duk_check_type_mask(ctx,
                                 index,
                                 DUK_TYPE_MASK_BOOLEAN | DUK_TYPE_MASK_NUMBER |
                                 DUK_TYPE_MASK_STRING |
                                 DUK_TYPE_MASK_OBJECT |
                                 DUK_TYPE_MASK_BUFFER |
                                 DUK_TYPE_MASK_POINTER |
                                 DUK_TYPE_MASK_LIGHTFUNC)
}

const DUK_BUF_MODE_FIXED: duktape::duk_uint_t = 0;
const DUK_BUF_MODE_DYNAMIC: duktape::duk_uint_t = 1;
const DUK_BUF_MODE_DONTCARE: duktape::duk_uint_t = 2;

#[inline(always)]
pub unsafe fn duk_to_buffer(ctx: *mut duktape::duk_context,
                            index: duktape::duk_idx_t,
                            out_size: *mut duktape::duk_size_t)
                            -> *mut c_void {
    duktape::duk_to_buffer_raw(ctx, index, out_size, DUK_BUF_MODE_DONTCARE)
}

#[inline(always)]
pub unsafe fn duk_to_fixed_buffer(ctx: *mut duktape::duk_context,
                                  index: duktape::duk_idx_t,
                                  out_size: *mut duktape::duk_size_t)
                                  -> *mut c_void {
    duktape::duk_to_buffer_raw(ctx, index, out_size, DUK_BUF_MODE_FIXED)
}

#[inline(always)]
pub unsafe fn duk_to_dynamic_buffer(ctx: *mut duktape::duk_context,
                                    index: duktape::duk_idx_t,
                                    out_size: *mut duktape::duk_size_t)
                                    -> *mut c_void {
    duktape::duk_to_buffer_raw(ctx, index, out_size, DUK_BUF_MODE_DYNAMIC)
}

#[inline(always)]
pub unsafe fn duk_safe_to_string(ctx: *mut duktape::duk_context,
                                 index: duktape::duk_idx_t)
                                 -> *const c_char {
    duktape::duk_safe_to_lstring(ctx, index, ptr::null_mut())
}

#[inline(always)]
pub unsafe fn duk_eval(ctx: *mut duktape::duk_context) {
    duktape::duk_eval_raw(ctx,
                          ptr::null(),
                          0,
                          2 | DUK_COMPILE_EVAL | DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_eval_noresult(ctx: *mut duktape::duk_context) {
    duktape::duk_eval_raw(ctx,
                          ptr::null(),
                          0,
                          2 | DUK_COMPILE_EVAL | DUK_COMPILE_NORESULT | DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_peval(ctx: *mut duktape::duk_context) -> duktape::duk_int_t {
    duktape::duk_eval_raw(ctx,
                          ptr::null(),
                          0,
                          2 | DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_peval_noresult(ctx: *mut duktape::duk_context) -> duktape::duk_int_t {
    duktape::duk_eval_raw(ctx,
                          ptr::null(),
                          0,
                          2 | DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NORESULT |
                          DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_compile(ctx: *mut duktape::duk_context, flags: duktape::duk_uint_t) {
    duktape::duk_compile_raw(ctx, ptr::null(), 0, 3 | flags);
}

#[inline(always)]
pub unsafe fn duk_pcompile(ctx: *mut duktape::duk_context,
                           flags: duktape::duk_uint_t)
                           -> duktape::duk_int_t {
    duktape::duk_compile_raw(ctx, ptr::null(), 0, 3 | flags | DUK_COMPILE_SAFE)
}

#[inline(always)]
pub unsafe fn duk_eval_string(ctx: *mut duktape::duk_context, src: *const c_char) {
    duktape::duk_eval_raw(ctx,
                          src,
                          0,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN |
                          DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_eval_string_noresult(ctx: *mut duktape::duk_context, src: *const c_char) {
    duktape::duk_eval_raw(ctx,
                          src,
                          0,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN |
                          DUK_COMPILE_NORESULT | DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_peval_string(ctx: *mut duktape::duk_context,
                               src: *const c_char)
                               -> duktape::duk_int_t {
    duktape::duk_eval_raw(ctx,
                          src,
                          0,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE |
                          DUK_COMPILE_STRLEN | DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_peval_string_noresult(ctx: *mut duktape::duk_context,
                                        src: *const c_char)
                                        -> duktape::duk_int_t {
    duktape::duk_eval_raw(ctx,
                          src,
                          0,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_SAFE | DUK_COMPILE_NOSOURCE |
                          DUK_COMPILE_STRLEN | DUK_COMPILE_NORESULT |
                          DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_compile_string(ctx: *mut duktape::duk_context,
                                 flags: duktape::duk_uint_t,
                                 src: *const c_char) {
    duktape::duk_compile_raw(ctx,
                             src,
                             0,
                             2 | flags | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN |
                             DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_compile_string_filename(ctx: *mut duktape::duk_context,
                                          flags: duktape::duk_uint_t,
                                          src: *const c_char) {
    duktape::duk_compile_raw(ctx,
                             src,
                             0,
                             2 | flags | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN);
}

#[inline(always)]
pub unsafe fn duk_pcompile_string(ctx: *mut duktape::duk_context,
                                  flags: duktape::duk_uint_t,
                                  src: *const c_char)
                                  -> duktape::duk_int_t {
    duktape::duk_compile_raw(ctx,
                             src,
                             0,
                             2 | flags | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN |
                             DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_pcompile_string_filename(ctx: *mut duktape::duk_context,
                                           flags: duktape::duk_uint_t,
                                           src: *const c_char)
                                           -> duktape::duk_int_t {
    duktape::duk_compile_raw(ctx,
                             src,
                             0,
                             2 | flags | DUK_COMPILE_NOSOURCE | DUK_COMPILE_STRLEN)
}

#[inline(always)]
pub unsafe fn duk_eval_lstring(ctx: *mut duktape::duk_context,
                               buf: *const c_char,
                               len: duktape::duk_size_t) {
    duktape::duk_eval_raw(ctx,
                          buf,
                          len,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_eval_lstring_noresult(ctx: *mut duktape::duk_context,
                                        buf: *const c_char,
                                        len: duktape::duk_size_t) {
    duktape::duk_eval_raw(ctx,
                          buf,
                          len,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NORESULT |
                          DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_peval_lstring(ctx: *mut duktape::duk_context,
                                buf: *const c_char,
                                len: duktape::duk_size_t)
                                -> duktape::duk_int_t {
    duktape::duk_eval_raw(ctx,
                          buf,
                          len,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_peval_lstring_noresult(ctx: *mut duktape::duk_context,
                                         buf: *const c_char,
                                         len: duktape::duk_size_t)
                                         -> duktape::duk_int_t {
    duktape::duk_eval_raw(ctx,
                          buf,
                          len,
                          1 | DUK_COMPILE_EVAL | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NORESULT |
                          DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_compile_lstring(ctx: *mut duktape::duk_context,
                                  flags: duktape::duk_uint_t,
                                  buf: *const c_char,
                                  len: duktape::duk_size_t) {
    duktape::duk_compile_raw(ctx,
                             buf,
                             len,
                             1 | flags | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NOFILENAME);
}

#[inline(always)]
pub unsafe fn duk_compile_lstring_filename(ctx: *mut duktape::duk_context,
                                           flags: duktape::duk_uint_t,
                                           buf: *const c_char,
                                           len: duktape::duk_size_t) {
    duktape::duk_compile_raw(ctx, buf, len, 1 | flags | DUK_COMPILE_NOSOURCE);
}

#[inline(always)]
pub unsafe fn duk_pcompile_lstring(ctx: *mut duktape::duk_context,
                                   flags: duktape::duk_uint_t,
                                   buf: *const c_char,
                                   len: duktape::duk_size_t)
                                   -> duktape::duk_int_t {
    duktape::duk_compile_raw(ctx,
                             buf,
                             len,
                             1 | flags | DUK_COMPILE_NOSOURCE | DUK_COMPILE_NOFILENAME)
}

#[inline(always)]
pub unsafe fn duk_pcompile_lstring_filename(ctx: *mut duktape::duk_context,
                                            flags: duktape::duk_uint_t,
                                            buf: *const c_char,
                                            len: duktape::duk_size_t)
                                            -> duktape::duk_int_t {
    duktape::duk_compile_raw(ctx, buf, len, 1 | flags | DUK_COMPILE_NOSOURCE)
}

#[inline(always)]
pub unsafe fn duk_eval_file(ctx: *mut duktape::duk_context, path: *const c_char) {
    duktape::duk_push_string_file_raw(ctx, path, 0);
    duktape::duk_push_string(ctx, path);
    duktape::duk_eval_raw(ctx, ptr::null(), 0, 3 | DUK_COMPILE_EVAL);
}

#[inline(always)]
pub unsafe fn duk_eval_file_noresult(ctx: *mut duktape::duk_context, path: *const c_char) {
    duktape::duk_push_string_file_raw(ctx, path, 0);
    duktape::duk_push_string(ctx, path);
    duktape::duk_eval_raw(ctx,
                          ptr::null(),
                          0,
                          3 | DUK_COMPILE_EVAL | DUK_COMPILE_NORESULT);
}

#[inline(always)]
pub unsafe fn duk_peval_file(ctx: *mut duktape::duk_context,
                             path: *const c_char)
                             -> duktape::duk_int_t {
    duktape::duk_push_string_file_raw(ctx, path, 0);
    duktape::duk_push_string(ctx, path);
    duktape::duk_eval_raw(ctx, ptr::null(), 0, 3 | DUK_COMPILE_EVAL)
}

#[inline(always)]
pub unsafe fn duk_peval_file_noresult(ctx: *mut duktape::duk_context,
                                      path: *const c_char)
                                      -> duktape::duk_int_t {
    duktape::duk_push_string_file_raw(ctx, path, 0);
    duktape::duk_push_string(ctx, path);
    duktape::duk_eval_raw(ctx,
                          ptr::null(),
                          0,
                          3 | DUK_COMPILE_EVAL | DUK_COMPILE_NORESULT)
}

#[inline(always)]
pub unsafe fn duk_compile_file(ctx: *mut duktape::duk_context,
                               flags: duktape::duk_uint_t,
                               path: *const c_char) {
    duktape::duk_push_string_file_raw(ctx, path, 0);
    duktape::duk_push_string(ctx, path);
    duktape::duk_compile_raw(ctx, ptr::null(), 0, 3 | flags);
}

#[inline(always)]
pub unsafe fn duk_pcompile_file(ctx: *mut duktape::duk_context,
                                flags: duktape::duk_uint_t,
                                path: *const c_char)
                                -> duktape::duk_int_t {
    duktape::duk_push_string_file_raw(ctx, path, 0);
    duktape::duk_push_string(ctx, path);
    duktape::duk_compile_raw(ctx, ptr::null(), 0, 3 | flags)
}

#[inline(always)]
pub unsafe fn duk_debugger_attach(ctx: *mut duktape::duk_context,
                                  read_cb: duktape::duk_debug_read_function,
                                  write_cb: duktape::duk_debug_write_function,
                                  peek_cb: duktape::duk_debug_peek_function,
                                  read_flush_cb: duktape::duk_debug_read_flush_function,
                                  write_flush_cb: duktape::duk_debug_write_flush_function,
                                  detached_cb: duktape::duk_debug_detached_function,
                                  udata: *mut c_void) {
    duktape::duk_debugger_attach_custom(ctx,
                                        read_cb,
                                        write_cb,
                                        peek_cb,
                                        read_flush_cb,
                                        write_flush_cb,
                                        None,
                                        detached_cb,
                                        udata);
}
