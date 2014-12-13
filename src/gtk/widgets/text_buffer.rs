// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

use std::ptr;

use gtk::{mod, ffi};
use gtk::ffi::FFIWidget;
use gtk::cast::GTK_TEXT_BUFFER;

/// GtkTextBuffer — Stores attributed text for display in a GtkTextView

struct_Widget!(TextBuffer)

impl TextBuffer {
    pub fn new(text_tag_table: Option<gtk::TextTagTable>) -> Option<TextBuffer> {
        let tmp_pointer = unsafe {
            match text_tag_table {
                Some(ttl) => ffi::gtk_text_buffer_new(ttl.get_pointer()),
                None      => ffi::gtk_text_buffer_new(ptr::null_mut())
            }
        };

        check_pointer!(tmp_pointer, TextBuffer)
    }

    pub fn apply_tag(&self, tag: &gtk::TextTag, start: &gtk::TextIter, end: &gtk::TextIter) {
        unsafe {
            ffi::gtk_text_buffer_apply_tag(GTK_TEXT_BUFFER(self.get_widget()), tag.get_pointer(),
                start.get_pointer() as *const ffi::C_GtkTextIter, end.get_pointer() as *const ffi::C_GtkTextIter);
        };
    }
    
    pub fn insert(&self, iter: &gtk::TextIter, text: &str, len: i32) {
        unsafe { text.with_c_str(|c_str| {
            ffi::gtk_text_buffer_insert(GTK_TEXT_BUFFER(self.get_widget()), iter.get_pointer(), c_str,len as ::libc::c_int)
            })
        };
    }

    pub fn create_tag(&self, tag_name: &str, one: &str, two: &str) -> gtk::TextTag {
        let tmp_pointer = unsafe {
            tag_name.with_c_str(|c_tag_name| {
                one.with_c_str(|c_one| {
                    two.with_c_str (|c_two| {
                        ffi::gtk_text_buffer_create_tag(GTK_TEXT_BUFFER(self.get_widget()), c_tag_name, c_one, c_two, ptr::null::<::libc::c_char>())
                    })
                })
            })
        };

        gtk::TextTag::wrap_pointer(tmp_pointer)
   }

   pub fn create_tag_int(&self, tag_name: &str, one: &str, two: i32) -> gtk::TextTag {
        let tmp_pointer = unsafe {
            tag_name.with_c_str(|c_tag_name| {
                one.with_c_str(|c_one| {
                    ffi::gtk_text_buffer_create_tag(GTK_TEXT_BUFFER(self.get_widget()), c_tag_name, c_one, two, ptr::null::<::libc::c_char>())
                })
            })
        };

        gtk::TextTag::wrap_pointer(tmp_pointer)
    }

   pub fn create_tag_double(&self, tag_name: &str, one: &str, two: f64) -> gtk::TextTag {

        let tmp_pointer = unsafe {
            tag_name.with_c_str(|c_tag_name| {
                one.with_c_str(|c_one| {
                    ffi::gtk_text_buffer_create_tag(GTK_TEXT_BUFFER(self.get_widget()), c_tag_name, c_one, two, ptr::null::<::libc::c_char>())
                })
            })
        };

        gtk::TextTag::wrap_pointer(tmp_pointer)
    }


   pub fn get_iter_at_offset(&self, iter: &gtk::TextIter, char_offset: i32) {
        unsafe {
            ffi::gtk_text_buffer_get_iter_at_offset(GTK_TEXT_BUFFER(self.get_widget()), iter.get_pointer(), char_offset as ::libc::c_int)
        };
    }

}

impl_drop!(TextBuffer)
impl_TraitWidget!(TextBuffer)

impl gtk::TextBufferTrait for TextBuffer {}

impl_widget_events!(TextBuffer)