pub type Vertex = [f32; 3];
pub type Index = u32;
pub type TextureCoords = [f32; 2];
pub type Normals = [f32; 3];

#[derive(Debug)]
pub struct Collection {
	pub name: String,
	pub vertices: Vec<Vertex>,
	pub indices: Vec<Index>,
	pub normals: Vec<Normals>,
	pub texture_coordinates: Vec<TextureCoords>,
	pub matrix: nalgebra::Matrix4<f32>,
}

pub struct Loader {
	pub path: String,
	pub format: ModelFormat
}

#[derive(Debug)]
pub enum ModelFormat {
	OBJ = 0,
	GLTF = 1,
	GLB = 2,
	USDZ = 3,
	STL = 4,
	DAE = 5,
}

impl Loader {
	pub fn new(path: String, format: ModelFormat) -> Self {
		Loader {
			path,
			format
		}
	}
}

impl Loader {
	pub fn load(&mut self) -> Vec<Collection> {
		match self.format {
			ModelFormat::GLTF | ModelFormat::GLB => self.load_khronos_format(),
			ModelFormat::OBJ => self.load_as_obj(),
			_ => panic!("Loading {:#?} is not supported yet", self.format)
			// ModelFormat::DAE => self.load_as_dae(),
			// ModelFormat::STL => self.load_as_stl(),
			// ModelFormat::USDZ => self.load_as_usdz(),
		}
	}
}