use bevy::{prelude::{Component, Handle, Mesh}, render::mesh::{PrimitiveTopology, Indices}};
use crate::cp437::string_to_cp437;

#[derive(Component)]
pub struct SimpleConsoleMarker(pub usize);

pub(crate) struct SimpleConsole {
    pub(crate) font_index: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) terminal: Vec<u16>,
    pub(crate) mesh_handle: Option<Handle<Mesh>>
}


const SIZE_TMP : f32 = 8.0;

impl SimpleConsole {
    pub fn new(font_index: usize, width: usize, height: usize) -> Self {
        Self {
            font_index, width, height, terminal: vec![65; width*height],
            mesh_handle: None,
        }
    }

    fn texture_coords(&self, glyph: u16, chars_per_row: u16, n_rows: u16) -> [f32;4] {
        let base_x = glyph % chars_per_row;
        let base_y = glyph / n_rows;
        let scale_x = 1.0 / chars_per_row as f32;
        let scale_y = 1.0 / n_rows as f32;
        return [
            base_x as f32 * scale_x,
            base_y as f32 * scale_y,
            (base_x+1) as f32 * scale_x,
            (base_y+1) as f32 * scale_y,
        ];
    }

    pub fn build_mesh(&self, chars_per_row: u16, n_rows: u16) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(self.width * self.height * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(self.width * self.height * 6);
        let mut index_count = 0;
        let half_height = self.height as f32 / 2.0;
        let half_width = self.width as f32 / 2.0;
        for y in 0..self.height {
            let screen_y = (y as f32 - half_height) * SIZE_TMP;
            let mut idx = (self.height-1 -y) * self.width;
            for x in 0..self.width {
                let screen_x = (x as f32 - half_width) * SIZE_TMP;
                vertices.push([ screen_x, screen_y, 0.0 ]);
                vertices.push([ screen_x + SIZE_TMP, screen_y, 0.0 ]);
                vertices.push([ screen_x, screen_y + SIZE_TMP, 0.0 ]);
                vertices.push([ screen_x + SIZE_TMP, screen_y + SIZE_TMP, 0.0 ]);
                for _ in 0..4 {
                    normals.push([0.0, 1.0, 0.0]);
                }
                let tex = self.texture_coords(self.terminal[idx], chars_per_row, n_rows);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);

                // Not convinced this does anything at all
                colors.push([1.0, 0.0, 0.0, 1.0]);
                colors.push([1.0, 0.0, 0.0, 1.0]);
                colors.push([1.0, 0.0, 0.0, 1.0]);
                colors.push([1.0, 0.0, 0.0, 1.0]);

                indices.push(index_count);
                indices.push(index_count+1);
                indices.push(index_count+2);

                indices.push(index_count+3);
                indices.push(index_count+2);
                indices.push(index_count+1);

                index_count += 4;
                idx += 1;
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        //mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    pub fn build_uvs(&self, chars_per_row: u16, n_rows: u16) -> Vec<[f32; 2]> {
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        for y in 0..self.height {
            let mut idx = y * self.width;
            for _ in 0..self.width {
                let tex = self.texture_coords(self.terminal[idx], chars_per_row, n_rows);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);
                idx += 1;
            }
        }
        uv
    }

    pub fn cls(&mut self) {
        self.terminal.iter_mut().for_each(|c| *c = 32);
    }

    pub fn print<S: ToString>(&mut self, mut x: usize, y: usize, text: S) {
        let bytes = string_to_cp437(&text.to_string());
        for glyph in bytes {
            let idx = self.at(x, y);
            self.terminal[idx] = glyph;
            x += 1;
        }
    }

    fn at(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y) * self.width) + x
    }
}