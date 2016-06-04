use std;
use duktape::*;

macro_rules! make_ret {
    ($err:ident, $ret:ident) => {
        pub const $ret: duk_int_t = -$err;
    }
}

macro_rules! make_increasing_sequence {
    ($t:ty, $val: expr) => {};

    ($t:ty, $val:expr, $arg:ident) => {
        pub const $arg: $t = $val;
    };

    ($t:ty, $val:expr, $arg: ident, $($args:ident),+) => {
        make_increasing_sequence!($t, $val, $arg);
        make_increasing_sequence!($t, $val+1, $($args),+);
    }
}

macro_rules! make_pow2_sequence {
    ($t:ty, $val:expr) => {};

    ($t:ty, $val:expr, $arg:ident) => {
        pub const $arg: $t = (1 << ($val));
    };

    ($t:ty, $val:expr, $arg:ident, $($args:ident),+) => {
        make_pow2_sequence!($t, $val, $arg);
        make_pow2_sequence!($t, $val+1, $($args),+);
    }
}

pub const DUK_VERSION: std::os::raw::c_long = 10500;

pub const DUK_GIT_COMMIT: &'static str = "83d557704ee63f68ab40b6fcb00995c9b3d6777c";
pub const DUK_GIT_DESCRIBE: &'static str = "v1.5.0";
pub const DUK_GIT_BRANCH: &'static str = "master";

pub const DUK_DEBUG_PROTOCOL_VERSION: duk_int_t = 1;

pub const DUK_INVALID_INDEX: duk_int_t = std::i32::MIN;

pub const DUK_VARARGS: duk_int_t = -1;

pub const DUK_TYPE_MIN: duk_int_t = 0;
make_increasing_sequence!(duk_int_t,
                          0,
                          DUK_TYPE_NONE,
                          DUK_TYPE_UNDEFINED,
                          DUK_TYPE_NULL,
                          DUK_TYPE_BOOLEAN,
                          DUK_TYPE_NUMBER,
                          DUK_TYPE_STRING,
                          DUK_TYPE_OBJECT,
                          DUK_TYPE_BUFFER,
                          DUK_TYPE_POINTER,
                          DUK_TYPE_LIGHTFUNC);
pub const DUK_TYPE_MAX: duk_int_t = 9;

make_pow2_sequence!(duk_uint_t,
                    0,
                    DUK_TYPE_MASK_NONE,
                    DUK_TYPE_MASK_UNDEFINED,
                    DUK_TYPE_MASK_NULL,
                    DUK_TYPE_MASK_BOOLEAN,
                    DUK_TYPE_MASK_NUMBER,
                    DUK_TYPE_MASK_STRING,
                    DUK_TYPE_MASK_OBJECT,
                    DUK_TYPE_MASK_BUFFER,
                    DUK_TYPE_MASK_POINTER,
                    DUK_TYPE_MASK_LIGHTFUNC);

make_increasing_sequence!(duk_int_t,
                          0,
                          DUK_HINT_NONE,
                          DUK_HINT_STRING,
                          DUK_HINT_NUMBER);

make_pow2_sequence!(duk_uint_t,
                    0,
                    DUK_ENUM_INCLUDE_NONENUMERABLE,
                    DUK_ENUM_INCLUDE_INTERNAL,
                    DUK_ENUM_OWN_PROPERTIES_ONLY,
                    DUK_ENUM_ARRAY_INDICES_ONLY,
                    DUK_ENUM_SORT_ARRAY_INDICES,
                    DUK_ENUM_NO_PROXY_BEHAVIOR);

make_pow2_sequence!(duk_uint_t,
                    3,
                    DUK_COMPILE_EVAL,
                    DUK_COMPILE_FUNCTION,
                    DUK_COMPILE_STRICT,
                    DUK_COMPILE_SAFE,
                    DUK_COMPILE_NORESULT,
                    DUK_COMPILE_NOSOURCE,
                    DUK_COMPILE_STRLEN,
                    DUK_COMPILE_NOFILENAME);

make_pow2_sequence!(duk_uint_t,
                    0,
                    DUK_DEFPROP_WRITABLE,
                    DUK_DEFPROP_ENUMERABLE,
                    DUK_DEFPROP_CONFIGURABLE,
                    DUK_DEFPROP_HAVE_WRITABLE,
                    DUK_DEFPROP_HAVE_ENUMERABLE,
                    DUK_DEFPROP_HAVE_CONFIGURABLE,
                    DUK_DEFPROP_HAVE_VALUE,
                    DUK_DEFPROP_HAVE_GETTER,
                    DUK_DEFPROP_HAVE_SETTER,
                    DUK_DEFPROP_FORCE);

pub const DUK_DEFPROP_SET_WRITABLE: duk_uint_t = DUK_DEFPROP_HAVE_WRITABLE | DUK_DEFPROP_WRITABLE;
pub const DUK_DEFPROP_CLEAR_WRITABLE: duk_uint_t = DUK_DEFPROP_HAVE_WRITABLE;
pub const DUK_DEFPROP_SET_ENUMERABLE: duk_uint_t = DUK_DEFPROP_HAVE_ENUMERABLE |
                                                   DUK_DEFPROP_ENUMERABLE;
pub const DUK_DEFPROP_CLEAR_ENUMERABLE: duk_uint_t = DUK_DEFPROP_HAVE_ENUMERABLE;
pub const DUK_DEFPROP_SET_CONFIGURABLE: duk_uint_t = DUK_DEFPROP_HAVE_CONFIGURABLE |
                                                     DUK_DEFPROP_CONFIGURABLE;
pub const DUK_DEFPROP_CLEAR_CONFIGURABLE: duk_uint_t = DUK_DEFPROP_HAVE_CONFIGURABLE;

pub const DUK_THREAD_NEW_GLOBAL_ENV: duk_uint_t = (1 << 0);

pub const DUK_STRING_PUSH_SAFE: duk_uint_t = (1 << 0);

pub const DUK_ERR_NONE: duk_int_t = 0;

make_increasing_sequence!(duk_int_t,
                          50,
                          DUK_ERR_UNIMPLEMENTED_ERROR,
                          DUK_ERR_UNSUPPORTED_ERROR,
                          DUK_ERR_INTERNAL_ERROR,
                          DUK_ERR_ALLOC_ERROR,
                          DUK_ERR_ASSERTION_ERROR,
                          DUK_ERR_API_ERROR,
                          DUK_ERR_UNCAUGHT_ERROR);

make_increasing_sequence!(duk_int_t,
                          100,
                          DUK_ERR_ERROR,
                          DUK_ERR_EVAL_ERROR,
                          DUK_ERR_RANGE_ERROR,
                          DUK_ERR_REFERENCE_ERROR,
                          DUK_ERR_SYNTAX_ERROR,
                          DUK_ERR_TYPE_ERROR,
                          DUK_ERR_URI_ERROR);

make_ret!(DUK_ERR_UNIMPLEMENTED_ERROR, DUK_RET_UNIMPLEMENTED_ERROR);
make_ret!(DUK_ERR_UNSUPPORTED_ERROR, DUK_RET_UNSUPPORTED_ERROR);
make_ret!(DUK_ERR_INTERNAL_ERROR, DUK_RET_INTERNAL_ERROR);
make_ret!(DUK_ERR_ALLOC_ERROR, DUK_RET_ALLOC_ERROR);
make_ret!(DUK_ERR_ASSERTION_ERROR, DUK_RET_ASSERTION_ERROR);
make_ret!(DUK_ERR_API_ERROR, DUK_RET_API_ERROR);
make_ret!(DUK_ERR_UNCAUGHT_ERROR, DUK_RET_UNCAUGHT_ERROR);
make_ret!(DUK_ERR_ERROR, DUK_RET_ERROR);
make_ret!(DUK_ERR_EVAL_ERROR, DUK_RET_EVAL_ERROR);
make_ret!(DUK_ERR_RANGE_ERROR, DUK_RET_RANGE_ERROR);
make_ret!(DUK_ERR_REFERENCE_ERROR, DUK_RET_REFERENCE_ERROR);
make_ret!(DUK_ERR_SYNTAX_ERROR, DUK_RET_SYNTAX_ERROR);
make_ret!(DUK_ERR_TYPE_ERROR, DUK_RET_TYPE_ERROR);
make_ret!(DUK_ERR_URI_ERROR, DUK_RET_URI_ERROR);

make_increasing_sequence!(duk_uint_t, 0, DUK_EXEC_SUCCESS, DUK_EXEC_ERROR);

make_increasing_sequence!(duk_uint_t,
                          0,
                          DUK_LOG_TRACE,
                          DUK_LOG_DEBUG,
                          DUK_LOG_INFO,
                          DUK_LOG_WARN,
                          DUK_LOG_ERROR,
                          DUK_LOG_FATAL);
