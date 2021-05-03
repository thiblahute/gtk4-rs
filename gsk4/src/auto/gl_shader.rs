// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::GLUniformType;
use crate::Renderer;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;
use std::ptr;

glib::wrapper! {
    pub struct GLShader(Object<ffi::GskGLShader, ffi::GskGLShaderClass>);

    match fn {
        type_ => || ffi::gsk_gl_shader_get_type(),
    }
}

impl GLShader {
    #[doc(alias = "gsk_gl_shader_new_from_bytes")]
    pub fn from_bytes(sourcecode: &glib::Bytes) -> GLShader {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_gl_shader_new_from_bytes(
                sourcecode.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_new_from_resource")]
    pub fn from_resource(resource_path: &str) -> GLShader {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gsk_gl_shader_new_from_resource(
                resource_path.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_compile")]
    pub fn compile<P: IsA<Renderer>>(&self, renderer: &P) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::gsk_gl_shader_compile(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gsk_gl_shader_find_uniform_by_name")]
    pub fn find_uniform_by_name(&self, name: &str) -> i32 {
        unsafe {
            ffi::gsk_gl_shader_find_uniform_by_name(self.to_glib_none().0, name.to_glib_none().0)
        }
    }

    //#[doc(alias = "gsk_gl_shader_format_args")]
    //pub fn format_args(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<glib::Bytes> {
    //    unsafe { TODO: call ffi:gsk_gl_shader_format_args() }
    //}

    #[doc(alias = "gsk_gl_shader_get_arg_bool")]
    pub fn arg_bool(&self, args: &glib::Bytes, idx: i32) -> bool {
        unsafe {
            from_glib(ffi::gsk_gl_shader_get_arg_bool(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_float")]
    pub fn arg_float(&self, args: &glib::Bytes, idx: i32) -> f32 {
        unsafe {
            ffi::gsk_gl_shader_get_arg_float(self.to_glib_none().0, args.to_glib_none().0, idx)
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_int")]
    pub fn arg_int(&self, args: &glib::Bytes, idx: i32) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_arg_int(self.to_glib_none().0, args.to_glib_none().0, idx) }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_uint")]
    pub fn arg_uint(&self, args: &glib::Bytes, idx: i32) -> u32 {
        unsafe {
            ffi::gsk_gl_shader_get_arg_uint(self.to_glib_none().0, args.to_glib_none().0, idx)
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_vec2")]
    pub fn arg_vec2(&self, args: &glib::Bytes, idx: i32, out_value: &mut graphene::Vec2) {
        unsafe {
            ffi::gsk_gl_shader_get_arg_vec2(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_vec3")]
    pub fn arg_vec3(&self, args: &glib::Bytes, idx: i32, out_value: &mut graphene::Vec3) {
        unsafe {
            ffi::gsk_gl_shader_get_arg_vec3(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "gsk_gl_shader_get_arg_vec4")]
    pub fn arg_vec4(&self, args: &glib::Bytes, idx: i32, out_value: &mut graphene::Vec4) {
        unsafe {
            ffi::gsk_gl_shader_get_arg_vec4(
                self.to_glib_none().0,
                args.to_glib_none().0,
                idx,
                out_value.to_glib_none_mut().0,
            );
        }
    }

    #[doc(alias = "gsk_gl_shader_get_args_size")]
    pub fn args_size(&self) -> usize {
        unsafe { ffi::gsk_gl_shader_get_args_size(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_get_n_textures")]
    pub fn n_textures(&self) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_n_textures(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_get_n_uniforms")]
    pub fn n_uniforms(&self) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_n_uniforms(self.to_glib_none().0) }
    }

    #[doc(alias = "gsk_gl_shader_get_resource")]
    pub fn resource(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_get_resource(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_get_source")]
    pub fn source(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(ffi::gsk_gl_shader_get_source(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_gl_shader_get_uniform_name")]
    pub fn uniform_name(&self, idx: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gsk_gl_shader_get_uniform_name(
                self.to_glib_none().0,
                idx,
            ))
        }
    }

    #[doc(alias = "gsk_gl_shader_get_uniform_offset")]
    pub fn uniform_offset(&self, idx: i32) -> i32 {
        unsafe { ffi::gsk_gl_shader_get_uniform_offset(self.to_glib_none().0, idx) }
    }

    #[doc(alias = "gsk_gl_shader_get_uniform_type")]
    pub fn uniform_type(&self, idx: i32) -> GLUniformType {
        unsafe {
            from_glib(ffi::gsk_gl_shader_get_uniform_type(
                self.to_glib_none().0,
                idx,
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct GLShaderBuilder {
    resource: Option<String>,
    source: Option<glib::Bytes>,
}

impl GLShaderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> GLShader {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref resource) = self.resource {
            properties.push(("resource", resource));
        }
        if let Some(ref source) = self.source {
            properties.push(("source", source));
        }
        glib::Object::new::<GLShader>(&properties)
            .expect("Failed to create an instance of GLShader")
    }

    pub fn resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn source(mut self, source: &glib::Bytes) -> Self {
        self.source = Some(source.clone());
        self
    }
}

impl fmt::Display for GLShader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLShader")
    }
}
