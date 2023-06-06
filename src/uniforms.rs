use glam::{UVec2, Vec3};
use nannou;
use nannou::wgpu;
use nannou::wgpu::util::DeviceExt;

use crate::camera::Camera;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    world: glam::Mat4,
    view: glam::Mat4,
    proj: glam::Mat4,
}

impl Uniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { nannou::wgpu::bytes::from(self) }
    }

    pub(crate) fn new_as_buffer(
        window_size: glam::UVec2,
        camera: &Camera,
        device: &nannou::wgpu::Device,
    ) -> wgpu::Buffer {
        let uniforms = Uniforms::new(window_size, camera.calc_view_matrix());
        let uniforms_bytes = uniforms.as_bytes();
        let usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;

        device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: Some("Uniform buffer"),
            contents: uniforms_bytes,
            usage,
        })
    }

    pub(crate) fn new_as_buffer_view(
        window_size: glam::UVec2,
        camera: &Camera,
        device: &nannou::wgpu::Device,
    ) -> wgpu::Buffer {
        let uniforms = Uniforms::new(window_size, camera.calc_view_matrix().into());
        let uniforms_bytes = uniforms.as_bytes();
        let usage = wgpu::BufferUsages::COPY_SRC;

        device.create_buffer_init(&wgpu::BufferInitDescriptor {
            label: None,
            contents: uniforms_bytes,
            usage,
        })
    }

    pub fn new(size: UVec2, view: glam::Mat4) -> Uniforms {
        let rotation = glam::Mat4::from_rotation_y(0f32);
        let aspect_ratio = size.x as f32 / size.y as f32;
        let fov_y = std::f32::consts::FRAC_PI_2;
        let near = 0.0001;
        let far = 100.0;
        let proj = glam::Mat4::perspective_rh_gl(fov_y, aspect_ratio, near, far);
        let scale = glam::Mat4::from_scale(Vec3::splat(0.01));
        Uniforms {
            world: rotation,
            view: (view * scale).into(),
            proj: proj.into(),
        }
    }
}
