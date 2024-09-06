use nalgebra::Matrix4;
use crate::loaders::{Collection, Loader};

impl Loader {
    pub fn load_khronos_format(&mut self) -> Vec<Collection> {
        let (document, buffers, _) = gltf::import(self.path.clone()).unwrap_or_else(|_| panic!("Failed to read {}, Is it gltf/glb formatted", self.path));

        // Vector to hold collections for each mesh
        let mut collections = Vec::new();

        for mesh in document.meshes() {
            let mut collection = Collection {
				name: mesh.name().unwrap_or("unknown_mesh_name").to_owned(),
                vertices: Vec::new(),
                indices: Vec::new(),
                texture_coordinates: Vec::new(),
                normals: Vec::new(),
                matrix: Matrix4::identity(), // Default matrix
            };

            for primitive in mesh.primitives() {
                let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                if let Some(positions) = reader.read_positions() {
                    collection.vertices.extend(positions.map(|pos| {
                        [pos[0], pos[1], pos[2]]
                    }));
                }

                if let Some(tex_coords) = reader.read_tex_coords(0) {
                    collection.texture_coordinates.extend(tex_coords.into_f32().map(|tc| {
                        [tc[0], tc[1]]
                    }));
                }

                if let Some(indices) = reader.read_indices() {
                    collection.indices.extend(indices.into_u32());
                }

                if let Some(normals) = reader.read_normals() {
                    collection.normals.extend(normals.map(|normal| {
                        [normal[0], normal[1], normal[2]]
                    }));
                }
            }

            // Extract node matrices if needed, adjust if required
			let node = document.nodes().find(|node| 
				node.name().unwrap_or("") == collection.name.as_str()
			);
            if let Some(node) = node {
                let transform = node.transform().matrix();
                let flat_matrix: Vec<f32> = transform.iter().flat_map(|row| row.iter()).cloned().collect();
                collection.matrix = Matrix4::from_row_slice(&flat_matrix);
            }

            collections.push(collection);
        }

        collections
    }
}