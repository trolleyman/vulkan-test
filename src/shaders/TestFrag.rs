
pub struct TestFrag {
    shader: ::std::sync::Arc<::vulkano::shader::ShaderModule>,
}

impl TestFrag {
    /// Loads the shader in Vulkan as a `ShaderModule`.
    #[inline]
    pub fn load(device: &::std::sync::Arc<::vulkano::device::Device>)
                -> Result<TestFrag, ::vulkano::OomError>
    {

        
        unsafe {
            let data = [3, 2, 35, 7, 0, 0, 1, 0, 1, 0, 8, 0, 13, 0, 0, 0, 0, 0, 0, 0, 17, 0, 2, 0, 1, 0, 0, 0, 11, 0, 6, 0, 1, 0, 0, 0, 71, 76, 83, 76, 46, 115, 116, 100, 46, 52, 53, 48, 0, 0, 0, 0, 14, 0, 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 15, 0, 6, 0, 4, 0, 0, 0, 4, 0, 0, 0, 109, 97, 105, 110, 0, 0, 0, 0, 9, 0, 0, 0, 16, 0, 3, 0, 4, 0, 0, 0, 7, 0, 0, 0, 3, 0, 3, 0, 2, 0, 0, 0, 194, 1, 0, 0, 4, 0, 9, 0, 71, 76, 95, 65, 82, 66, 95, 115, 101, 112, 97, 114, 97, 116, 101, 95, 115, 104, 97, 100, 101, 114, 95, 111, 98, 106, 101, 99, 116, 115, 0, 0, 4, 0, 9, 0, 71, 76, 95, 65, 82, 66, 95, 115, 104, 97, 100, 105, 110, 103, 95, 108, 97, 110, 103, 117, 97, 103, 101, 95, 52, 50, 48, 112, 97, 99, 107, 0, 5, 0, 4, 0, 4, 0, 0, 0, 109, 97, 105, 110, 0, 0, 0, 0, 5, 0, 4, 0, 9, 0, 0, 0, 102, 95, 99, 111, 108, 111, 114, 0, 71, 0, 4, 0, 9, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 19, 0, 2, 0, 2, 0, 0, 0, 33, 0, 3, 0, 3, 0, 0, 0, 2, 0, 0, 0, 22, 0, 3, 0, 6, 0, 0, 0, 32, 0, 0, 0, 23, 0, 4, 0, 7, 0, 0, 0, 6, 0, 0, 0, 4, 0, 0, 0, 32, 0, 4, 0, 8, 0, 0, 0, 3, 0, 0, 0, 7, 0, 0, 0, 59, 0, 4, 0, 8, 0, 0, 0, 9, 0, 0, 0, 3, 0, 0, 0, 43, 0, 4, 0, 6, 0, 0, 0, 10, 0, 0, 0, 0, 0, 128, 63, 43, 0, 4, 0, 6, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 44, 0, 7, 0, 7, 0, 0, 0, 12, 0, 0, 0, 10, 0, 0, 0, 11, 0, 0, 0, 11, 0, 0, 0, 10, 0, 0, 0, 54, 0, 5, 0, 2, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 248, 0, 2, 0, 5, 0, 0, 0, 62, 0, 3, 0, 9, 0, 0, 0, 12, 0, 0, 0, 253, 0, 1, 0, 56, 0, 1, 0];

            Ok(TestFrag {
                shader: try!(::vulkano::shader::ShaderModule::new(device, &data))
            })
        }
    }

    /// Returns the module that was created.
    #[allow(dead_code)]
    #[inline]
    pub fn module(&self) -> &::std::sync::Arc<::vulkano::shader::ShaderModule> {
        &self.shader
    }
        
    /// Returns a logical struct describing the entry point named `main`.
    #[inline]
    pub fn main_entry_point(&self) -> ::vulkano::shader::FragmentShaderEntryPoint<([f32; 4],), Layout> {
        unsafe {
            #[allow(dead_code)]
            static NAME: [u8; 5] = [109, 97, 105, 110, 0];     // "main"
            self.shader.fragment_shader_entry_point(::std::ffi::CStr::from_ptr(NAME.as_ptr() as *const _), Layout)
        }
    }
            
}
        pub mod ty {}
#[derive(Default)]
pub struct Layout;

unsafe impl ::vulkano::descriptor_set::Layout for Layout {
    type DescriptorSets = ();
    type DescriptorSetLayouts = ();
    type PushConstants = ();

    fn decode_descriptor_set_layouts(&self, layouts: Self::DescriptorSetLayouts)
        -> Vec<::std::sync::Arc<::vulkano::descriptor_set::AbstractDescriptorSetLayout>>
    {
        vec![
            
        ]
    }

    fn decode_descriptor_sets(&self, sets: Self::DescriptorSets)
        -> Vec<::std::sync::Arc<::vulkano::descriptor_set::AbstractDescriptorSet>>
    {
        vec![
            
        ]
    }
}
