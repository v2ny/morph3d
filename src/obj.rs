use tobj::{self, LoadOptions};
use nalgebra::Matrix4;
use crate::loaders::{Collection, Loader};

impl Loader {
    pub fn load_as_obj(&mut self) -> Vec<Collection> {
        // Load the OBJ file using tobj
        let (models, _) = tobj::load_obj(self.path.clone(), &LoadOptions {
            triangulate: true,
            ..Default::default()
        }).expect("Failed to load OBJ file");

        // Vector to hold collections for each mesh
        let mut collections = Vec::new();
		let cloned_path = self.path.clone();

        for model in models {
            let mesh = model.mesh;
            let vertices = &mesh.positions;
            let indices = &mesh.indices;
            let normals = &mesh.normals;

            let collection = Collection {
				name: cloned_path.rsplit_once("/").expect("bad path").1.replace(".obj", "").replace(' ', "").to_string(),
                vertices: vertices.chunks(3).map(|chunk| [chunk[0], chunk[1], chunk[2]]).collect(),
                indices: indices.to_vec(),
                texture_coordinates: mesh.texcoords.chunks(2).map(|chunk| [chunk[0], chunk[1]]).collect(),
                normals: normals.chunks(3).map(|chunk| [chunk[0], chunk[1], chunk[2]]).collect(),
                matrix: Matrix4::identity(), // Default matrix, adjust if needed
            };

            collections.push(collection);
        }

        collections
    }
}