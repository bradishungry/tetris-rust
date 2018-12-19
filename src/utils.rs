use std::fs::File;
use std::io::Read;
use wavefront_obj;

use hal::{
    Backend,
    buffer,
    Device,
    format::{Aspects, Format, Swizzle},
    image::{self as img, ViewCapabilities},
    memory::Properties,
    MemoryType,
};

#[derive(Debug, Clone, Copy)]
pub struct PushConstants {
    pub tint: [f32; 4],
    pub position: [f32; 3],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UniformBlock {
    pub projection: [[f32; 4]; 4],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub uv: [f32; 2],
}

pub const MESH: &[Vertex] = &[
    Vertex {
        position: [0.0, -1.0, 0.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 0.0, 0.0],
        color: [0.0, 0.0, 1.0, 1.0],
        uv: [0.0, 0.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [0.0, -1.0, 0.0],
        color: [1.0, 0.0, 0.0, 1.0],
        uv: [1.0, 0.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        color: [0.0, 1.0, 0.0, 1.0],
        uv: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        color: [1.0, 1.0, 0.0, 1.0],
        uv: [1.0, 1.0],
    },
    ];

pub fn load(fname: String) -> Vec<Vertex> {
    let meshes = wavefront_obj::obj::parse(fname).unwrap();
    let (vertices, uvs) = (
        meshes.objects[0].vertices.clone(),
        meshes.objects[0].tex_vertices.clone() //TODO: Pointer to vertices? Move and destroy?
        );
    let mut converted_vertices: Vec<Vertex> = 
        vertices.into_iter().map(|vert|
            Vertex { 
                position: [vert.x as f32, vert.y as f32, vert.z as f32],
                color: [1.0, 1.0, 1.0, 1.0],
                uv: [1.0, 1.0],
            }
        ).collect();

    converted_vertices
}

pub fn load_obj(fname: String) -> Vec<Vertex> {
    let mut f = File::open(fname).expect("File not found!!");
    let mut file_content = String::new();
    f.read_to_string(&mut file_content).expect("Read in error");

    let words: Vec<Vec<&str>> = file_content.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect();
    let scale = 0.27;

    let mut verticies: Vec<Vertex> = Vec::new();
    let mut sorted_verticies: Vec<Vertex> = Vec::new();
    let mut temp_vertex: Vertex;
    for line in words {
        //println!("{:?}", line[0]);
        if line.len() == 0 { continue; }
        match line[0] {
            "v" =>  {

                let x = line[1].parse::<f32>().unwrap() * scale;
                let y = line[2].parse::<f32>().unwrap() * scale - 0.4;
                let z = line[3].parse::<f32>().unwrap() * scale;

                temp_vertex = Vertex {
                    position: [x, y * 1.2 + 1.0, z],
                    color: [1.0, 0.2, 1.0, 1.0],
                    uv: [1.0, 1.0],
                };

                verticies.push(temp_vertex);
            },

            "f" => {
                sorted_verticies.push(
                    verticies[line[1].parse::<i32>().unwrap() as usize]
                    );
                sorted_verticies.push(
                    verticies[line[2].parse::<i32>().unwrap() as usize]
                    );
                sorted_verticies.push(
                    verticies[line[3].parse::<i32>().unwrap() as usize]
                    );
            },
            _ => (),
        };
    }

    if !sorted_verticies.is_empty(){
        for v in sorted_verticies {

        }
    }

    verticies

}

pub fn empty_buffer<B: Backend, Item>(    
    device: &B::Device,
    memory_types: &[MemoryType],
    properties: Properties,
    usage: buffer::Usage,
    item_count: usize,
    ) -> (B::Buffer, B::Memory) {

    let item_count = item_count;     
    let stride = ::std::mem::size_of::<Item>() as u64;
    let buffer_len = item_count as u64 * stride;
    let unbound_buffer = device.create_buffer(buffer_len, usage).unwrap();
    let req = device.get_buffer_requirements(&unbound_buffer);
    let upload_type = memory_types
        .iter()
        .enumerate()
        .position(|(id, ty)| req.type_mask & (1 << id) != 0 && ty.properties.contains(properties))
        .unwrap()
        .into();

    let buffer_memory = device.allocate_memory(upload_type, req.size).unwrap();
    let buffer = device
        .bind_buffer_memory(&buffer_memory, 0, unbound_buffer)
        .unwrap();

    (buffer, buffer_memory)
}

/// Pushes data into a buffer.
pub fn fill_buffer<B: Backend, Item: Copy>(
    device: &B::Device,
    buffer_memory: &mut B::Memory,
    items: &[Item],
    ) {

    let stride = ::std::mem::size_of::<Item>() as u64;
    let buffer_len = items.len() as u64 * stride;

    let mut dest = device
        .acquire_mapping_writer::<Item>(&buffer_memory, 0..buffer_len)
        .unwrap();
    dest.copy_from_slice(items);
    device.release_mapping_writer(dest);
}

/// Creates a buffer and immediately fills it.
pub fn create_buffer<B: Backend, Item: Copy>(
    device: &B::Device,
    memory_types: &[MemoryType],
    properties: Properties,
    usage: buffer::Usage,
    items: &[Item],
    ) -> (B::Buffer, B::Memory) {
    let (empty_buffer, mut empty_buffer_memory) =
        empty_buffer::<B, Item>(device, memory_types, properties, usage, items.len());

    fill_buffer::<B, Item>(device, &mut empty_buffer_memory, items);

    (empty_buffer, empty_buffer_memory)
}

/// Reinterpret an instance of T as a slice of u32s that can be uploaded as push
/// constants.
pub fn push_constant_data<T>(data: &T) -> &[u32] {
    let size = push_constant_size::<T>();
    let ptr = data as *const T as *const u32;

    unsafe { ::std::slice::from_raw_parts(ptr, size) }
}

/// Determine the number of push constants required to store T.
/// Panics if T is not a multiple of 4 bytes - the size of a push constant.
pub fn push_constant_size<T>() -> usize {
    const PUSH_CONSTANT_SIZE: usize = ::std::mem::size_of::<u32>();
    let type_size = ::std::mem::size_of::<T>();

    // We want to ensure that the type we upload as a series of push constants
    // is actually representable as a series of u32 push constants.
    assert!(type_size % PUSH_CONSTANT_SIZE == 0);

    type_size / PUSH_CONSTANT_SIZE
}

pub fn create_image<B: Backend>(
    device: &B::Device,
    memory_types: &[MemoryType],
    width: u32,
    height: u32,
    format: Format,
    usage: img::Usage,
    aspects: Aspects,
    ) -> (B::Image, B::Memory, B::ImageView) {
    let kind = img::Kind::D2(width, height, 1, 1);

    let unbound_image = device
        .create_image(
            kind,
            1,
            format,
            img::Tiling::Optimal,
            usage,
            ViewCapabilities::empty(),
            ).expect("Failed to create unbound image");

    let image_req = device.get_image_requirements(&unbound_image);

    let device_type = memory_types
        .iter()
        .enumerate()
        .position(|(id, memory_type)| {
            image_req.type_mask & (1 << id) != 0
                && memory_type.properties.contains(Properties::DEVICE_LOCAL)
        }).unwrap()
    .into();

    let image_memory = device
        .allocate_memory(device_type, image_req.size)
        .expect("Failed to allocate image");

    let image = device
        .bind_image_memory(&image_memory, 0, unbound_image)
        .expect("Failed to bind image");

    let image_view = device
        .create_image_view(
            &image,
            img::ViewKind::D2,
            format,
            Swizzle::NO,
            img::SubresourceRange {
                aspects,
                levels: 0..1,
                layers: 0..1,
            },
            ).expect("Failed to create image view");

    (image, image_memory, image_view)
}


