/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Widget {
    _unused: [u8; 0],
}
pub type Fl_Callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut Fl_Widget, arg2: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn Fl_Widget_callback_with_captures(
        arg1: *mut Fl_Widget,
        cb: Fl_Callback,
        arg2: *mut ::std::os::raw::c_void,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Output {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Output_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Output;
}
extern "C" {
    pub fn Fl_Output_x(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_y(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_width(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_height(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_label(arg1: *mut Fl_Output) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Output_set_label(arg1: *mut Fl_Output, title: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Output_redraw(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_show(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_hide(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_activate(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_deactivate(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_redraw_label(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_resize(
        arg1: *mut Fl_Output,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Output_tooltip(arg1: *mut Fl_Output) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Output_set_tooltip(arg1: *mut Fl_Output, txt: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn Fl_Output_get_type(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_type(arg1: *mut Fl_Output, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_color(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_color(arg1: *mut Fl_Output, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_label_color(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_label_color(arg1: *mut Fl_Output, color: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_label_font(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_label_font(arg1: *mut Fl_Output, font: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_label_size(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_label_size(arg1: *mut Fl_Output, sz: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_label_type(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_label_type(arg1: *mut Fl_Output, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_box(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_box(arg1: *mut Fl_Output, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_changed(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_changed(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_clear_changed(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_align(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_align(arg1: *mut Fl_Output, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_delete(arg1: *mut Fl_Output);
}
extern "C" {
    pub fn Fl_Output_set_value(
        arg1: *mut Fl_Output,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_value(arg1: *mut Fl_Output) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Output_maximum_size(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_maximum_size(arg1: *mut Fl_Output, m: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_position(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_position(
        arg1: *mut Fl_Output,
        p: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_mark(
        arg1: *mut Fl_Output,
        m: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_mark(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_replace(
        arg1: *mut Fl_Output,
        b: ::std::os::raw::c_int,
        e: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_insert(
        arg1: *mut Fl_Output,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_append(
        arg1: *mut Fl_Output,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        keep_selection: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_copy(
        arg1: *mut Fl_Output,
        clipboard: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_undo(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_copy_cuts(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_text_font(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_text_font(arg1: *mut Fl_Output, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_text_color(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_text_color(arg1: *mut Fl_Output, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_text_size(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_text_size(arg1: *mut Fl_Output, s: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_readonly(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_readonly(arg1: *mut Fl_Output, boolean: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Output_wrap(arg1: *mut Fl_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Output_set_wrap(arg1: *mut Fl_Output, boolean: ::std::os::raw::c_int);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Fl_Multiline_Output {
    _unused: [u8; 0],
}
extern "C" {
    pub fn Fl_Multiline_Output_new(
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        title: *const ::std::os::raw::c_char,
    ) -> *mut Fl_Multiline_Output;
}
extern "C" {
    pub fn Fl_Multiline_Output_x(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_y(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_width(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_height(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_label(
        arg1: *mut Fl_Multiline_Output,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_label(
        arg1: *mut Fl_Multiline_Output,
        title: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_redraw(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_show(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_hide(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_activate(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_deactivate(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_redraw_label(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_resize(
        arg1: *mut Fl_Multiline_Output,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_tooltip(
        arg1: *mut Fl_Multiline_Output,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_tooltip(
        arg1: *mut Fl_Multiline_Output,
        txt: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_get_type(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_type(arg1: *mut Fl_Multiline_Output, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multiline_Output_color(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_color(
        arg1: *mut Fl_Multiline_Output,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_label_color(arg1: *mut Fl_Multiline_Output)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_label_color(
        arg1: *mut Fl_Multiline_Output,
        color: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_label_font(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_label_font(
        arg1: *mut Fl_Multiline_Output,
        font: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_label_size(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_label_size(
        arg1: *mut Fl_Multiline_Output,
        sz: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_label_type(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_label_type(
        arg1: *mut Fl_Multiline_Output,
        typ: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_box(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_box(arg1: *mut Fl_Multiline_Output, typ: ::std::os::raw::c_int);
}
extern "C" {
    pub fn Fl_Multiline_Output_changed(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_changed(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_clear_changed(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_align(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_align(
        arg1: *mut Fl_Multiline_Output,
        typ: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_delete(arg1: *mut Fl_Multiline_Output);
}
extern "C" {
    pub fn Fl_Multiline_Output_set_value(
        arg1: *mut Fl_Multiline_Output,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_value(
        arg1: *mut Fl_Multiline_Output,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn Fl_Multiline_Output_maximum_size(
        arg1: *mut Fl_Multiline_Output,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_maximum_size(
        arg1: *mut Fl_Multiline_Output,
        m: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_position(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_position(
        arg1: *mut Fl_Multiline_Output,
        p: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_mark(
        arg1: *mut Fl_Multiline_Output,
        m: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_mark(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_replace(
        arg1: *mut Fl_Multiline_Output,
        b: ::std::os::raw::c_int,
        e: ::std::os::raw::c_int,
        text: *const ::std::os::raw::c_char,
        ilen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_insert(
        arg1: *mut Fl_Multiline_Output,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_append(
        arg1: *mut Fl_Multiline_Output,
        t: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        keep_selection: ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_copy(
        arg1: *mut Fl_Multiline_Output,
        clipboard: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_undo(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_copy_cuts(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_text_font(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_text_font(
        arg1: *mut Fl_Multiline_Output,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_text_color(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_text_color(
        arg1: *mut Fl_Multiline_Output,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_text_size(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_text_size(
        arg1: *mut Fl_Multiline_Output,
        s: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_readonly(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_readonly(
        arg1: *mut Fl_Multiline_Output,
        boolean: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn Fl_Multiline_Output_wrap(arg1: *mut Fl_Multiline_Output) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn Fl_Multiline_Output_set_wrap(
        arg1: *mut Fl_Multiline_Output,
        boolean: ::std::os::raw::c_int,
    );
}
