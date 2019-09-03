// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct FilterListModel(Object<gtk_sys::GtkFilterListModel, gtk_sys::GtkFilterListModelClass, FilterListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || gtk_sys::gtk_filter_list_model_get_type(),
    }
}

impl FilterListModel {
    pub fn new<P: IsA<gio::ListModel>>(
        model: &P,
        filter_func: Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>,
    ) -> FilterListModel {
        assert_initialized_main_thread!();
        let filter_func_data: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            Box_::new(filter_func);
        unsafe extern "C" fn filter_func_func<P: IsA<gio::ListModel>>(
            item: *mut gobject_sys::GObject,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let item = from_glib_borrow(item);
            let callback: &Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&item)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter_func = if filter_func_data.is_some() {
            Some(filter_func_func::<P> as _)
        } else {
            None
        };
        unsafe extern "C" fn user_destroy_func<P: IsA<gio::ListModel>>(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(user_destroy_func::<P> as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            filter_func_data;
        unsafe {
            from_glib_full(gtk_sys::gtk_filter_list_model_new(
                model.as_ref().to_glib_none().0,
                filter_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            ))
        }
    }

    pub fn new_for_type(item_type: glib::types::Type) -> FilterListModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_filter_list_model_new_for_type(
                item_type.to_glib(),
            ))
        }
    }
}

pub const NONE_FILTER_LIST_MODEL: Option<&FilterListModel> = None;

pub trait FilterListModelExt: 'static {
    fn get_model(&self) -> Option<gio::ListModel>;

    fn has_filter(&self) -> bool;

    fn refilter(&self);

    fn set_filter_func(&self, filter_func: Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>);

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>);

    fn get_property_has_filter(&self) -> bool;

    fn get_property_item_type(&self) -> glib::types::Type;

    fn connect_property_has_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FilterListModel>> FilterListModelExt for O {
    fn get_model(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_filter_list_model_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn has_filter(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_filter_list_model_has_filter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn refilter(&self) {
        unsafe {
            gtk_sys::gtk_filter_list_model_refilter(self.as_ref().to_glib_none().0);
        }
    }

    fn set_filter_func(&self, filter_func: Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>) {
        let filter_func_data: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            Box_::new(filter_func);
        unsafe extern "C" fn filter_func_func(
            item: *mut gobject_sys::GObject,
            user_data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let item = from_glib_borrow(item);
            let callback: &Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>> =
                &*(user_data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&item)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let filter_func = if filter_func_data.is_some() {
            Some(filter_func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn user_destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
                Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(user_destroy_func as _);
        let super_callback0: Box_<Option<Box_<dyn Fn(&glib::Object) -> bool + 'static>>> =
            filter_func_data;
        unsafe {
            gtk_sys::gtk_filter_list_model_set_filter_func(
                self.as_ref().to_glib_none().0,
                filter_func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_filter_list_model_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn get_property_has_filter(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"has-filter\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `has-filter` getter")
                .unwrap()
        }
    }

    fn get_property_item_type(&self) -> glib::types::Type {
        unsafe {
            let mut value = Value::from_type(<glib::types::Type as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"item-type\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `item-type` getter")
                .unwrap()
        }
    }

    fn connect_property_has_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_filter_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFilterListModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FilterListModel>,
        {
            let f: &F = &*(f as *const F);
            f(&FilterListModel::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-filter\0".as_ptr() as *const _,
                Some(transmute(notify_has_filter_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FilterListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FilterListModel")
    }
}
