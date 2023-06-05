use crate::obj::parser::Triangle;
use crate::obj::solver::solve_indices;
use crate::obj::{Indices, Normals, Vertices};

use glam::Vec3A;

pub struct Mesh {
    pub(crate) triangles: Vec<Triangle>,
    pub(crate) normals: Normals,
    pub(crate) calculated: Normals,
    pub(crate) vertices: Vertices,
    pub(crate) uvs: Vertices,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            triangles: vec![],
            normals: vec![],
            calculated: vec![],
            vertices: vec![],
            uvs: vec![],
        }
    }

    pub fn as_buffers(&mut self) -> (Indices, Vertices, Vertices, Normals) {
        let (vp, uv, nm, faces) =
            solve_indices(&self.vertices, &self.uvs, &self.normals, &self.triangles);
        (faces.iter().map(|x| *x as u16).collect(), vp, uv, nm)
    }

    pub(crate) fn normal_from_indexes(&self, triangle: &Triangle) -> Vec3A {
        Triangle::normal_from_points(
            self.vertices[triangle.points[0]],
            self.vertices[triangle.points[1]],
            self.vertices[triangle.points[2]],
        )
    }
}