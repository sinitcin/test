#![allow(
    dead_code,
    non_camel_case_types,
    non_upper_case_globals,
    non_snake_case
)]
// Const
static PATH_OWL: &str = "./data/owl.jpg";
static PATH_CROPOWL: &str = "./data/owl_crop.jpg";
static CSTRING_FAILED: &str = "CString::new - не смог создать строку.";
static ERROR_LOADING: &str = "загрузки";
static ERROR_CROP: &str = "обрезки";
static ERROR_SAVE: &str = "сохранения";

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::{io, ptr};

///
/// # Библиотека VIPS
/// Чтобы использовать VIPS нужно выполнить в терминале:
/// ```bash
/// $sudo apt-get install libvips*
/// ```
//  # Биндинги
//

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VipsBufferThread {
    pub hash: *mut GHashTable,
    pub thread: *mut GThread,
}

pub type VipsBufferCache = _VipsBufferCache;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsBufferCache {
    pub buffers: *mut GSList,
    pub thread: *mut GThread,
    pub im: *mut _VipsImage,
    pub buffer_thread: *mut VipsBufferThread,
    pub reserve: *mut GSList,
    pub n_reserve: ::std::os::raw::c_int,
}

pub type VipsBuffer = _VipsBuffer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsBuffer {
    pub ref_count: ::std::os::raw::c_int,
    pub im: *mut _VipsImage,
    pub area: VipsRect,
    pub done: gboolean,
    pub cache: *mut VipsBufferCache,
    pub buf: *mut VipsPel,
    pub bsize: usize,
}

pub type GObjectConstructParam = _GObjectConstructParam;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GObjectConstructParam {
    pub pspec: *mut GParamSpec,
    pub value: *mut GValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VipsWindow {
    pub ref_count: ::std::os::raw::c_int,
    pub im: *mut _VipsImage,
    pub top: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub data: *mut VipsPel,
    pub baseaddr: *mut ::std::os::raw::c_void,
    pub length: usize,
}

pub type VipsStartFn = ::std::option::Option<
    unsafe extern "C" fn(
        out: *mut _VipsImage,
        a: *mut ::std::os::raw::c_void,
        b: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type VipsStopFn = ::std::option::Option<
    unsafe extern "C" fn(
        seq: *mut ::std::os::raw::c_void,
        a: *mut ::std::os::raw::c_void,
        b: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type VipsGenerateFn = ::std::option::Option<
    unsafe extern "C" fn(
        out: *mut _VipsRegion,
        seq: *mut ::std::os::raw::c_void,
        a: *mut ::std::os::raw::c_void,
        b: *mut ::std::os::raw::c_void,
        stop: *mut gboolean,
    ) -> ::std::os::raw::c_int,
>;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GThreadPriority {
    G_THREAD_PRIORITY_LOW = 0,
    G_THREAD_PRIORITY_NORMAL = 1,
    G_THREAD_PRIORITY_HIGH = 2,
    G_THREAD_PRIORITY_URGENT = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum _RegionType {
    VIPS_REGION_NONE = 0,
    VIPS_REGION_BUFFER = 1,
    VIPS_REGION_OTHER_REGION = 2,
    VIPS_REGION_OTHER_IMAGE = 3,
    VIPS_REGION_WINDOW = 4,
}
pub use self::_RegionType as RegionType;

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum GParamFlags {
    G_PARAM_READABLE = 1,
    G_PARAM_WRITABLE = 2,
    G_PARAM_READWRITE = 3,
    G_PARAM_CONSTRUCT = 4,
    G_PARAM_CONSTRUCT_ONLY = 8,
    G_PARAM_LAX_VALIDATION = 16,
    G_PARAM_STATIC_NAME = 32,
    G_PARAM_STATIC_NICK = 64,
    G_PARAM_STATIC_BLURB = 128,
    G_PARAM_EXPLICIT_NOTIFY = 1073741824,
    G_PARAM_DEPRECATED = -2147483648,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VipsInterpretation {
    VIPS_INTERPRETATION_ERROR = -1,
    VIPS_INTERPRETATION_MULTIBAND = 0,
    VIPS_INTERPRETATION_B_W = 1,
    VIPS_INTERPRETATION_HISTOGRAM = 10,
    VIPS_INTERPRETATION_XYZ = 12,
    VIPS_INTERPRETATION_LAB = 13,
    VIPS_INTERPRETATION_CMYK = 15,
    VIPS_INTERPRETATION_LABQ = 16,
    VIPS_INTERPRETATION_RGB = 17,
    VIPS_INTERPRETATION_CMC = 18,
    VIPS_INTERPRETATION_LCH = 19,
    VIPS_INTERPRETATION_LABS = 21,
    VIPS_INTERPRETATION_sRGB = 22,
    VIPS_INTERPRETATION_YXY = 23,
    VIPS_INTERPRETATION_FOURIER = 24,
    VIPS_INTERPRETATION_RGB16 = 25,
    VIPS_INTERPRETATION_GREY16 = 26,
    VIPS_INTERPRETATION_MATRIX = 27,
    VIPS_INTERPRETATION_scRGB = 28,
    VIPS_INTERPRETATION_HSV = 29,
    VIPS_INTERPRETATION_LAST = 30,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VipsCoding {
    VIPS_CODING_ERROR = -1,
    VIPS_CODING_NONE = 0,
    VIPS_CODING_LABQ = 2,
    VIPS_CODING_RAD = 6,
    VIPS_CODING_LAST = 7,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VipsBandFormat {
    VIPS_FORMAT_NOTSET = -1,
    VIPS_FORMAT_UCHAR = 0,
    VIPS_FORMAT_CHAR = 1,
    VIPS_FORMAT_USHORT = 2,
    VIPS_FORMAT_SHORT = 3,
    VIPS_FORMAT_UINT = 4,
    VIPS_FORMAT_INT = 5,
    VIPS_FORMAT_FLOAT = 6,
    VIPS_FORMAT_COMPLEX = 7,
    VIPS_FORMAT_DOUBLE = 8,
    VIPS_FORMAT_DPCOMPLEX = 9,
    VIPS_FORMAT_LAST = 10,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VipsImageType {
    VIPS_IMAGE_ERROR = -1,
    VIPS_IMAGE_NONE = 0,
    VIPS_IMAGE_SETBUF = 1,
    VIPS_IMAGE_SETBUF_FOREIGN = 2,
    VIPS_IMAGE_OPENIN = 3,
    VIPS_IMAGE_MMAPIN = 4,
    VIPS_IMAGE_MMAPINRW = 5,
    VIPS_IMAGE_OPENOUT = 6,
    VIPS_IMAGE_PARTIAL = 7,
}

pub type GThread = _GThread;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GThread {
    pub func: GThreadFunc,
    pub data: gpointer,
    pub joinable: gboolean,
    pub priority: GThreadPriority,
}

pub type GThreadPool = _GThreadPool;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GThreadPool {
    pub func: GFunc,
    pub user_data: gpointer,
    pub exclusive: gboolean,
}

pub type GParamSpec = _GParamSpec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GParamSpec {
    pub g_type_instance: GTypeInstance,
    pub name: *const gchar,
    pub flags: GParamFlags,
    pub value_type: GType,
    pub owner_type: GType,
    pub _nick: *mut gchar,
    pub _blurb: *mut gchar,
    pub qdata: *mut GData,
    pub ref_count: guint,
    pub param_id: guint,
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VipsDemandStyle {
    VIPS_DEMAND_STYLE_ERROR = -1,
    VIPS_DEMAND_STYLE_SMALLTILE = 0,
    VIPS_DEMAND_STYLE_FATSTRIP = 1,
    VIPS_DEMAND_STYLE_THINSTRIP = 2,
    VIPS_DEMAND_STYLE_ANY = 3,
}

pub type VipsRect = _VipsRect;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsRect {
    pub left: ::std::os::raw::c_int,
    pub top: ::std::os::raw::c_int,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
}

pub type GTimer = _GTimer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GTimer {
    _unused: [u8; 0],
}

pub type GList = _GList;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}

pub type VipsProgress = _VipsProgress;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsProgress {
    pub im: *mut _VipsImage,
    pub run: ::std::os::raw::c_int,
    pub eta: ::std::os::raw::c_int,
    pub tpels: gint64,
    pub npels: gint64,
    pub percent: ::std::os::raw::c_int,
    pub start: *mut GTimer,
}

pub type VipsRegion = _VipsRegion;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsRegion {
    pub parent_object: VipsObject,
    pub im: *mut VipsImage,
    pub valid: VipsRect,
    pub type_: RegionType,
    pub data: *mut VipsPel,
    pub bpl: ::std::os::raw::c_int,
    pub seq: *mut ::std::os::raw::c_void,
    pub thread: *mut GThread,
    pub window: *mut VipsWindow,
    pub buffer: *mut VipsBuffer,
    pub invalid: gboolean,
}

pub type GMutex = _GMutex;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _GMutex {
    pub p: gpointer,
    pub i: [guint; 2usize],
    _bindgen_union_align: u64,
}

pub type GTypeClass = _GTypeClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GTypeClass {
    pub g_type: GType,
}

pub type GTypeInstance = _GTypeInstance;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GTypeInstance {
    pub g_class: *mut GTypeClass,
}

pub type GData = _GData;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GData {
    _unused: [u8; 0],
}

pub type GValue = _GValue;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _GValue {
    pub g_type: GType,
    pub data: [_GValue__data; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union _GValue__data {
    pub v_int: gint,
    pub v_uint: guint,
    pub v_long: glong,
    pub v_ulong: gulong,
    pub v_int64: gint64,
    pub v_uint64: guint64,
    pub v_float: gfloat,
    pub v_double: gdouble,
    pub v_pointer: gpointer,
    _union_align: u64,
}

pub type VipsBuf = _VipsBuf;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsBuf {
    pub base: *mut ::std::os::raw::c_char,
    pub mx: ::std::os::raw::c_int,
    pub i: ::std::os::raw::c_int,
    pub full: gboolean,
    pub lasti: ::std::os::raw::c_int,
    pub dynamic: gboolean,
}

pub type VipsArgumentTable = GHashTable;
pub type GHashTable = _GHashTable;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GHashTable {
    _unused: [u8; 0],
}

pub type GObjectClass = _GObjectClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GObjectClass {
    pub g_type_class: GTypeClass,
    pub construct_properties: *mut GSList,
    pub constructor: ::std::option::Option<
        unsafe extern "C" fn(
            type_: GType,
            n_construct_properties: guint,
            construct_properties: *mut GObjectConstructParam,
        ) -> *mut GObject,
    >,
    pub set_property: ::std::option::Option<
        unsafe extern "C" fn(
            object: *mut GObject,
            property_id: guint,
            value: *const GValue,
            pspec: *mut GParamSpec,
        ),
    >,
    pub get_property: ::std::option::Option<
        unsafe extern "C" fn(
            object: *mut GObject,
            property_id: guint,
            value: *mut GValue,
            pspec: *mut GParamSpec,
        ),
    >,
    pub dispose: ::std::option::Option<unsafe extern "C" fn(object: *mut GObject)>,
    pub finalize: ::std::option::Option<unsafe extern "C" fn(object: *mut GObject)>,
    pub dispatch_properties_changed: ::std::option::Option<
        unsafe extern "C" fn(object: *mut GObject, n_pspecs: guint, pspecs: *mut *mut GParamSpec),
    >,
    pub notify:
        ::std::option::Option<unsafe extern "C" fn(object: *mut GObject, pspec: *mut GParamSpec)>,
    pub constructed: ::std::option::Option<unsafe extern "C" fn(object: *mut GObject)>,
    pub flags: gsize,
    pub pdummy: [gpointer; 6usize],
}

pub type GObject = _GObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GObject {
    pub g_type_instance: GTypeInstance,
    pub ref_count: guint,
    pub qdata: *mut GData,
}

pub type GClosureNotifyData = _GClosureNotifyData;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GClosureNotifyData {
    pub data: gpointer,
    pub notify: GClosureNotify,
}

pub type GClosure = _GClosure;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GClosure {
    pub _bitfield_1: __BitfieldUnit<[u8; 4usize], u16>,
    pub marshal: ::std::option::Option<
        unsafe extern "C" fn(
            closure: *mut GClosure,
            return_value: *mut GValue,
            n_param_values: guint,
            param_values: *const GValue,
            invocation_hint: gpointer,
            marshal_data: gpointer,
        ),
    >,
    pub data: gpointer,
    pub notifiers: *mut GClosureNotifyData,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    storage: Storage,
    align: [Align; 0],
}

pub type VipsObject = _VipsObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsObject {
    pub parent_instance: GObject,
    pub constructed: gboolean,
    pub static_object: gboolean,
    pub argument_table: *mut VipsArgumentTable,
    pub nickname: *mut ::std::os::raw::c_char,
    pub description: *mut ::std::os::raw::c_char,
    pub preclose: gboolean,
    pub close: gboolean,
    pub postclose: gboolean,
    pub local_memory: usize,
}

pub type VipsObjectClass = _VipsObjectClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsObjectClass {
    pub parent_class: GObjectClass,
    pub build: ::std::option::Option<
        unsafe extern "C" fn(object: *mut VipsObject) -> ::std::os::raw::c_int,
    >,
    pub postbuild: ::std::option::Option<
        unsafe extern "C" fn(object: *mut VipsObject) -> ::std::os::raw::c_int,
    >,
    pub summary_class:
        ::std::option::Option<unsafe extern "C" fn(cls: *mut _VipsObjectClass, buf: *mut VipsBuf)>,
    pub summary:
        ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject, buf: *mut VipsBuf)>,
    pub dump:
        ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject, buf: *mut VipsBuf)>,
    pub sanity:
        ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject, buf: *mut VipsBuf)>,
    pub rewind: ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject)>,
    pub preclose: ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject)>,
    pub close: ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject)>,
    pub postclose: ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject)>,
    pub new_from_string: ::std::option::Option<
        unsafe extern "C" fn(string: *const ::std::os::raw::c_char) -> *mut VipsObject,
    >,
    pub to_string:
        ::std::option::Option<unsafe extern "C" fn(object: *mut VipsObject, buf: *mut VipsBuf)>,
    pub output_needs_arg: gboolean,
    pub output_to_arg: ::std::option::Option<
        unsafe extern "C" fn(
            object: *mut VipsObject,
            string: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub nickname: *const ::std::os::raw::c_char,
    pub description: *const ::std::os::raw::c_char,
    pub argument_table: *mut VipsArgumentTable,
    pub argument_table_traverse: *mut GSList,
    pub argument_table_traverse_gtype: GType,
    pub deprecated: gboolean,
    pub _vips_reserved1: ::std::option::Option<unsafe extern "C" fn()>,
    pub _vips_reserved2: ::std::option::Option<unsafe extern "C" fn()>,
    pub _vips_reserved3: ::std::option::Option<unsafe extern "C" fn()>,
    pub _vips_reserved4: ::std::option::Option<unsafe extern "C" fn()>,
}

pub type GFunc = ::std::option::Option<unsafe extern "C" fn(data: gpointer, user_data: gpointer)>;
pub type GThreadFunc = ::std::option::Option<unsafe extern "C" fn(data: gpointer) -> gpointer>;
pub type GClosureNotify =
    ::std::option::Option<unsafe extern "C" fn(data: gpointer, closure: *mut GClosure)>;

pub type GInitiallyUnowned = _GObject;
pub type GInitiallyUnownedClass = _GObjectClass;
pub type GType = gsize;
pub type gchararray = *mut gchar;
pub type GClosure_autoptr = *mut GClosure;
pub type GClosure_listautoptr = *mut GList;
pub type GClosure_slistautoptr = *mut GSList;
pub type GObject_autoptr = *mut GObject;
pub type GObject_listautoptr = *mut GList;
pub type GObject_slistautoptr = *mut GSList;
pub type GInitiallyUnowned_autoptr = *mut GInitiallyUnowned;
pub type GInitiallyUnowned_listautoptr = *mut GList;
pub type GInitiallyUnowned_slistautoptr = *mut GSList;
pub type VipsPel = ::std::os::raw::c_uchar;
pub type wchar_t = ::std::os::raw::c_int;
pub type gint8 = ::std::os::raw::c_schar;
pub type guint8 = ::std::os::raw::c_uchar;
pub type gint16 = ::std::os::raw::c_short;
pub type guint16 = ::std::os::raw::c_ushort;
pub type gint32 = ::std::os::raw::c_int;
pub type guint32 = ::std::os::raw::c_uint;
pub type gint64 = ::std::os::raw::c_long;
pub type guint64 = ::std::os::raw::c_ulong;
pub type gssize = ::std::os::raw::c_long;
pub type gsize = ::std::os::raw::c_ulong;
pub type goffset = gint64;
pub type gintptr = ::std::os::raw::c_long;
pub type guintptr = ::std::os::raw::c_ulong;
pub type GPid = ::std::os::raw::c_int;
pub type gchar = ::std::os::raw::c_char;
pub type gshort = ::std::os::raw::c_short;
pub type glong = ::std::os::raw::c_long;
pub type gint = ::std::os::raw::c_int;
pub type gboolean = gint;
pub type guchar = ::std::os::raw::c_uchar;
pub type gushort = ::std::os::raw::c_ushort;
pub type gulong = ::std::os::raw::c_ulong;
pub type guint = ::std::os::raw::c_uint;
pub type gfloat = f32;
pub type gdouble = f64;
pub type gpointer = *mut ::std::os::raw::c_void;
pub type gconstpointer = *const ::std::os::raw::c_void;

pub type GSList = _GSList;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GSList {
    pub data: gpointer,
    pub next: *mut GSList,
}

pub type VipsImage = _VipsImage;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _VipsImage {
    pub parent_instance: VipsObject,
    pub Xsize: ::std::os::raw::c_int,
    pub Ysize: ::std::os::raw::c_int,
    pub Bands: ::std::os::raw::c_int,
    pub BandFmt: VipsBandFormat,
    pub Coding: VipsCoding,
    pub Type: VipsInterpretation,
    pub Xres: f64,
    pub Yres: f64,
    pub Xoffset: ::std::os::raw::c_int,
    pub Yoffset: ::std::os::raw::c_int,
    pub Length: ::std::os::raw::c_int,
    pub Compression: ::std::os::raw::c_short,
    pub Level: ::std::os::raw::c_short,
    pub Bbits: ::std::os::raw::c_int,
    pub time: *mut VipsProgress,
    pub Hist: *mut ::std::os::raw::c_char,
    pub filename: *mut ::std::os::raw::c_char,
    pub data: *mut VipsPel,
    pub kill: ::std::os::raw::c_int,
    pub Xres_float: f32,
    pub Yres_float: f32,
    pub mode: *mut ::std::os::raw::c_char,
    pub dtype: VipsImageType,
    pub fd: ::std::os::raw::c_int,
    pub baseaddr: *mut ::std::os::raw::c_void,
    pub length: usize,
    pub magic: guint32,
    pub start_fn: VipsStartFn,
    pub generate_fn: VipsGenerateFn,
    pub stop_fn: VipsStopFn,
    pub client1: *mut ::std::os::raw::c_void,
    pub client2: *mut ::std::os::raw::c_void,
    pub sslock: *mut GMutex,
    pub regions: *mut GSList,
    pub dhint: VipsDemandStyle,
    pub meta: *mut GHashTable,
    pub meta_traverse: *mut GSList,
    pub sizeof_header: gint64,
    pub windows: *mut GSList,
    pub upstream: *mut GSList,
    pub downstream: *mut GSList,
    pub serial: ::std::os::raw::c_int,
    pub history_list: *mut GSList,
    pub progress_signal: *mut _VipsImage,
    pub file_length: gint64,
    pub hint_set: gboolean,
    pub delete_on_close: gboolean,
    pub delete_on_close_filename: *mut ::std::os::raw::c_char,
}

#[link(name = "vips")]
extern "C" {
    pub fn vips_image_new_from_file(filename: *const c_char, ...) -> *const VipsImage;
    pub fn vips_smartcrop(
        in_img: *mut VipsImage,
        out_img: *mut *mut VipsImage,
        width: c_int,
        height: c_int,
        ...
    ) -> c_int;
    pub fn vips_jpegsave(in_img: *mut VipsImage, filename: *const c_char, ...) -> c_int;
}

// Безопасный интерфейс
fn crop_image(from_file: &str, to_file: &str, width: i32, height: i32) -> io::Result<i32> {
    let image = unsafe {
        let result =
            vips_image_new_from_file(CString::new(from_file).expect(CSTRING_FAILED).as_ptr(), 0);
        // Не безопасное преобразование, по этому блок остался тут
        if result as usize == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Ошибка в процессе {} изображения!",
                    ERROR_LOADING
                ),
            ));
        };
        result
    } as *mut _;
    let mut data: *mut VipsImage = ptr::null_mut::<VipsImage>();
    let crop_result = unsafe { vips_smartcrop(image, &mut data, width, height, 0) };
    if crop_result != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Ошибка в процессе {} изображения!",
                ERROR_CROP
            ),
        ));
    };
    let save_result = unsafe {
        vips_jpegsave(
            data,
            CString::new(to_file).expect(CSTRING_FAILED).as_ptr(),
            0,
        )
    };
    if save_result != 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "Ошибка в процессе {} изображения!",
                ERROR_SAVE
            ),
        ));
    };
    Ok(0)
}

#[cfg(test)]
mod tests {

    use std::ffi::CString;
    use std::fs;
    use std::ptr;

    // Global
    static mut image: Option<super::VipsImage> = None;

    #[test]
    fn load_image() {
        unsafe {
            let vimage = super::vips_image_new_from_file(
                CString::new(super::PATH_OWL)
                    .expect(super::CSTRING_FAILED)
                    .as_ptr(),
                0,
            );
            assert_ne!(
                vimage as usize,
                0,
                "Ошибка в процессе {} изображения!",
                super::ERROR_LOADING
            );
            image = Some(*vimage);
        }
    }

    #[test]
    fn smart_crop() {
        unsafe {
            if let Some(ref mut vimage) = image {
                let mut data: *mut super::VipsImage = ptr::null_mut::<super::VipsImage>();
                let crop_result = super::vips_smartcrop(vimage, &mut data, 100, 100, 0);
                assert_eq!(
                    crop_result,
                    0,
                    "Ошибка в процессе {} изображения!",
                    super::ERROR_CROP
                );

                let save_result = super::vips_jpegsave(
                    data,
                    CString::new(super::PATH_CROPOWL)
                        .expect(super::CSTRING_FAILED)
                        .as_ptr(),
                    0,
                );
                assert_eq!(
                    save_result,
                    0,
                    "Ошибка в процессе {} изображения!",
                    super::ERROR_SAVE
                );

                fs::remove_file(super::PATH_CROPOWL).unwrap();
            } else {
                panic!("Нет открытых изображений.");
            }
        }
    }

    #[test]
    fn safe_iface() {
        super::crop_image(super::PATH_OWL, super::PATH_CROPOWL, 150, 150).unwrap();
    }
}
