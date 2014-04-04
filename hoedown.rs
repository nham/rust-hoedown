use std::libc;
use std::str;

static HOEDOWN_EXT_TABLES: libc::c_uint = 1 << 0;
static HOEDOWN_EXT_FENCED_CODE: libc::c_uint = 1 << 1;
static HOEDOWN_EXT_FOOTNOTES: libc::c_uint = 1 << 2;
static HOEDOWN_EXT_AUTOLINK: libc::c_uint = 1 << 3;
static HOEDOWN_EXT_STRIKETHROUGH: libc::c_uint = 1 << 4;
static HOEDOWN_EXT_UNDERLINE: libc::c_uint = 1 << 5;
static HOEDOWN_EXT_HIGHLIGHT: libc::c_uint = 1 << 6;
static HOEDOWN_EXT_QUOTE: libc::c_uint = 1 << 7;
static HOEDOWN_EXT_SUPERSCRIPT: libc::c_uint = 1 << 8;
static HOEDOWN_EXT_LAX_SPACING: libc::c_uint = 1 << 9;
static HOEDOWN_EXT_NO_INTRA_EMPHASIS: libc::c_uint = 1 << 10;
static HOEDOWN_EXT_SPACE_HEADERS: libc::c_uint = 1 << 11;
static HOEDOWN_EXT_DISABLE_INDENTED_CODE: libc::c_uint = 1 << 12;

// was buf in sundown
struct hoedown_buffer {
    data: *u8,
    size: libc::size_t,
    asize: libc::size_t,
    unit: libc::size_t,
}

// was sd_callbacks in sundown
struct hoedown_renderer {
    opaque: *libc::c_void,
    other: [libc::size_t, ..32],
}

// was sd_markdown in sundown
type hoedown_document = libc::c_void; // opaque


// ported from rust's sundown binding
#[link(name = "hoedown")]
extern {
    fn hoedown_html_renderer_new(render_flags: libc::c_uint,
                                 nesting_level: libc::c_int) -> *hoedown_renderer;

    fn hoedown_html_renderer_free(renderer: *hoedown_renderer);

    fn hoedown_document_render(doc: *hoedown_document,
                        ob: *hoedown_buffer,
                        document: *u8,
                        doc_size: libc::size_t);

    fn hoedown_document_new(renderer: *hoedown_renderer,
                            extensions: libc::c_uint,
                            max_nesting: libc::size_t) -> *hoedown_document;

    fn hoedown_document_free(doc: *hoedown_document);


    fn hoedown_buffer_new(unit: libc::size_t) -> *hoedown_buffer;

    fn hoedown_buffer_free(b: *hoedown_buffer);

    fn hoedown_buffer_puts(buf: *hoedown_buffer, c: *libc::c_char);
}


fn main() {
    unsafe {
        let extensions = 0;
        let renderer: *hoedown_renderer = hoedown_html_renderer_new(0, 0);
        let document: *hoedown_document = hoedown_document_new(renderer, extensions, 16);
        let html: *hoedown_buffer = hoedown_buffer_new(16);
        let s = ~"Hello, **world**";
        hoedown_document_render(document, html, s.as_ptr(), s.len() as libc::size_t);
        print!("{}", str::raw::from_buf_len((*html).data, (*html).size as uint));

        hoedown_buffer_free(html);
        hoedown_document_free(document);
        hoedown_html_renderer_free(renderer);
    }
}
