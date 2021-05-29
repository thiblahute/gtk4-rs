// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{RenderNodeType, RoundedRect};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OutsetShadowNode(Shared<ffi::GskOutsetShadowNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

define_render_node!(
    OutsetShadowNode,
    ffi::GskOutsetShadowNode,
    ffi::gsk_outset_shadow_node_get_type,
    RenderNodeType::OutsetShadowNode
);

impl OutsetShadowNode {
    #[doc(alias = "gsk_outset_shadow_node_new")]
    pub fn new(
        outline: &RoundedRect,
        color: &gdk::RGBA,
        dx: f32,
        dy: f32,
        spread: f32,
        blur_radius: f32,
    ) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_outset_shadow_node_new(
                outline.to_glib_none().0,
                color.to_glib_none().0,
                dx,
                dy,
                spread,
                blur_radius,
            ))
        }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_blur_radius")]
    #[doc(alias = "get_blur_radius")]
    pub fn blur_radius(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_blur_radius(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_color")]
    #[doc(alias = "get_color")]
    pub fn color(&self) -> Option<gdk::RGBA> {
        unsafe { from_glib_none(ffi::gsk_outset_shadow_node_get_color(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_dx")]
    #[doc(alias = "get_dx")]
    pub fn dx(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_dx(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_dy")]
    #[doc(alias = "get_dy")]
    pub fn dy(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_dy(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_outline")]
    #[doc(alias = "get_outline")]
    pub fn outline(&self) -> Option<RoundedRect> {
        unsafe {
            from_glib_none(ffi::gsk_outset_shadow_node_get_outline(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_outset_shadow_node_get_spread")]
    #[doc(alias = "get_spread")]
    pub fn spread(&self) -> f32 {
        unsafe { ffi::gsk_outset_shadow_node_get_spread(self.to_glib_none().0) }
    }
}
