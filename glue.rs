/* automatically generated by rust-bindgen */

use core::libc::*;
use jsapi::*;
use jsfriendapi::JSJitInfo;

pub type enum_StubType = c_uint;
pub const PROPERTY_STUB: u32 = 0_u32;
pub const STRICT_PROPERTY_STUB: u32 = 1_u32;
pub const ENUMERATE_STUB: u32 = 2_u32;
pub const CONVERT_STUB: u32 = 3_u32;
pub const RESOLVE_STUB: u32 = 4_u32;

pub struct ProxyTraps {
    getPropertyDescriptor: *u8,
    getOwnPropertyDescriptor: *u8,
    defineProperty: *u8,
    getOwnPropertyNames: *u8,
    delete_: *u8,
    enumerate: *u8,

    has: *u8,
    hasOwn: *u8,
    get: *u8,
    set: *u8,
    keys: *u8,
    iterate: *u8,

    call: *u8,
    construct: *u8,
    nativeCall: *u8,
    hasInstance: *u8,
    typeOf: *u8,
    objectClassIs: *u8,
    obj_toString: *u8,
    fun_toString: *u8,
    //regexp_toShared: *u8,
    defaultValue: *u8,
    iteratorNext: *u8,
    finalize: *u8,
    getElementIfPresent: *u8,
    getPrototypeOf: *u8
}

#[link_name="jsglue"]
pub extern mod bindgen {

#[rust_stack]
pub fn GetJSClassHookStubPointer(++_type: enum_StubType) -> *c_void;

#[rust_stack]
pub fn RUST_JSVAL_IS_NULL(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_IS_VOID(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_IS_INT(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_TO_INT(++v: JSVal) -> int32_t;

#[rust_stack]
pub fn RUST_INT_TO_JSVAL(++v: int32_t) -> JSVal;

#[rust_stack]
pub fn RUST_JSVAL_IS_DOUBLE(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_TO_DOUBLE(++v: JSVal) -> c_double;

#[rust_stack]
pub fn RUST_DOUBLE_TO_JSVAL(++v: c_double) -> JSVal;

#[rust_stack]
pub fn RUST_UINT_TO_JSVAL(++v: uint32_t) -> JSVal;

#[rust_stack]
pub fn RUST_JSVAL_IS_NUMBER(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_IS_STRING(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_TO_STRING(++v: JSVal) -> *JSString;

#[rust_stack]
pub fn RUST_STRING_TO_JSVAL(++v: *JSString) -> JSVal;

#[rust_stack]
pub fn RUST_JSVAL_TO_OBJECT(++v: JSVal) -> *JSObject;

#[rust_stack]
pub fn RUST_OBJECT_TO_JSVAL(++v: *JSObject) -> JSVal;

#[rust_stack]
pub fn RUST_JSVAL_IS_BOOLEAN(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_TO_BOOLEAN(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_BOOLEAN_TO_JSVAL(++v: JSBool) -> JSVal;

#[rust_stack]
pub fn RUST_JSVAL_IS_PRIMITIVE(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_IS_GCTHING(++v: JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_JSVAL_TO_GCTHING(++v: JSVal) -> *c_void;

#[rust_stack]
pub fn RUST_PRIVATE_TO_JSVAL(++v: *c_void) -> JSVal;

#[rust_stack]
pub fn RUST_JSVAL_TO_PRIVATE(++v: JSVal) -> *c_void;

#[rust_stack]
pub fn RUST_JS_NumberValue(++d: f64) -> JSVal;

//#[rust_stack]
pub fn CallJitPropertyOp(++info: *JSJitInfo, ++cx: *JSContext, ++thisObj: *JSObject, ++specializedThis: *libc::c_void, ++vp: *JSVal) -> JSBool;

//#[rust_stack]
pub fn CallJitMethodOp(++info: *JSJitInfo, ++cx: *JSContext, ++thisObj: *JSObject, ++specializedThis: *libc::c_void, ++argc: libc::c_uint, ++vp: *JSVal) -> JSBool;

#[rust_stack]
pub fn RUST_FUNCTION_VALUE_TO_JITINFO(++v: *JSVal) -> *JSJitInfo;

pub fn SetFunctionNativeReserved(fun: *JSObject, which: libc::size_t, val: *JSVal);
pub fn GetFunctionNativeReserved(fun: *JSObject, which: libc::size_t) -> *JSVal;

pub fn CreateProxyHandler(traps: *ProxyTraps) -> *libc::c_void;
pub fn NewProxyObject(cx: *JSContext, handler: *libc::c_void, priv_: *JSVal,
                      proto: *JSObject, parent: *JSObject, call: *JSObject,
                      construct: *JSObject) -> *JSObject;

pub fn GetProxyExtra(obj: *JSObject, slot: c_uint) -> JSVal;
pub fn GetProxyPrivate(obj: *JSObject) -> JSVal;

pub fn GetObjectProto(obj: *JSObject) -> *JSObject;

pub fn RUST_JSID_IS_INT(id: jsid) -> JSBool;
pub fn RUST_JSID_TO_INT(id: jsid) -> libc::c_int;

pub fn RUST_SET_JITINFO(func: *JSFunction, info: *JSJitInfo);

pub fn RUST_INTERNED_STRING_TO_JSID(cx: *JSContext, str: *JSString) -> jsid;

pub fn DefineFunctionWithReserved(cx: *JSContext, obj: *JSObject,
                                  name: *libc::c_char, call: JSNative, nargs: libc::c_uint,
                                  attrs: libc::c_uint) -> *JSObject;
pub fn GetObjectJSClass(obj: *JSObject) -> *JSClass;
pub fn js_GetErrorMessage(userRef: *libc::c_void, locale: *libc::c_char,
                          errorNumber: libc::c_uint) -> *JSErrorFormatString;
}
