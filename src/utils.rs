use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
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
                    position: [x, y + 0.5, z],
                    color: [1.0, 0.2, 1.0, 1.0],
                };

                verticies.push(temp_vertex);
            }
            _ => (),
        };
    }

    verticies

}
