//use std::collections::hash_map::HashMap;
use std::time::{Duration, Instant};

use winit;
use winit::WindowEvent::*;

//hal
use back;
use hal::{
    adapter::MemoryTypeId,
    buffer,
    command::{ClearColor, ClearValue},
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{Access, Layout, SubresourceRange, ViewKind},
    memory::Properties,
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    },
    pool::CommandPoolCreateFlags,
    pso::{
        AttributeDesc, BlendState, ColorBlendDesc, ColorMask, Element, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, VertexBufferDesc, Viewport,
    },
    queue::Submission,
    Backbuffer, Device, FrameSync, Graphics, Instance, PhysicalDevice, Primitive, Surface, SwapImageIndex,
    Swapchain, SwapchainConfig,
};

use utils::{self, Vertex};

const MESH: &[Vertex] = &[
    Vertex {
        position: [0.0, -1.0, 0.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 0.0, 0.0],
        color: [0.0, 0.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.0, -1.0, 0.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    Vertex {
        position: [0.0, 1.0, 0.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 0.0, 0.0],
        color: [1.0, 1.0, 0.0, 1.0],
    },
];

pub fn game_loop() {

    const FPS: u32 = 60;

    //winit
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::WindowBuilder::new()
        .with_title("Your faithful window")
        .with_dimensions((800, 600).into())
        .build(&events_loop)
        .unwrap();

    //let mut texcache: HashMap<String, Texture> = HashMap::new();

    let instance = back::Instance::create("Tetris", 1);
    let mut surface = instance.create_surface(&window);
    let mut adapter = instance.enumerate_adapters().remove(0);
    let (device, mut queue_group) = adapter
        .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
        .unwrap();

    let max_buffers = 16;
    let mut command_pool = device.create_command_pool_typed(
        &queue_group,
        CommandPoolCreateFlags::empty(),
        max_buffers,
        ).unwrap();

    let physical_device = &adapter.physical_device;
    let (_, formats, _) = surface.compatibility(physical_device);

    let surface_color_format = {
        // We must pick a color format from the list of supported formats. If there
        // is no list, we default to Rgba8Srgb.
        match formats {
            Some(choices) => choices
                .into_iter()
                .find(|format| format.base_format().1 == ChannelType::Srgb)
                .unwrap(),
            None => Format::Rgba8Srgb,
        }
    };

    let render_pass = {
        let color_attachment = Attachment {
            format: Some(surface_color_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::Present,
        };

        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        let dependency = SubpassDependency {
            passes: SubpassRef::External..SubpassRef::Pass(0),
            stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
            accesses: Access::empty()
                ..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
        };

        device.create_render_pass(&[color_attachment], &[subpass], &[dependency]).unwrap()
    };

    let pipeline_layout = device.create_pipeline_layout(&[], &[]).unwrap();

    let vertex_shader_module = {
        let spirv = include_bytes!("../assets/gen/shaders/basic.glslv.spv");
        device.create_shader_module(spirv).unwrap()
    };

    let fragment_shader_module = {
        let spirv = include_bytes!("../assets/gen/shaders/basic.glslf.spv");
        device.create_shader_module(spirv).unwrap()
    };

    let pipeline = {
        let vs_entry = EntryPoint::<back::Backend> {
            entry: "main",
            module: &vertex_shader_module,
            specialization: Default::default(),
        };

        let fs_entry = EntryPoint::<back::Backend> {
            entry: "main",
            module: &fragment_shader_module,
            specialization: Default::default(),
        };

        let shader_entries = GraphicsShaderSet {
            vertex: vs_entry,
            hull: None,
            domain: None,
            geometry: None,
            fragment: Some(fs_entry),
        };

        let subpass = Subpass {
            index: 0,
            main_pass: &render_pass,
        };

        let mut pipeline_desc = GraphicsPipelineDesc::new(
            shader_entries,
            Primitive::TriangleList,
            Rasterizer::FILL,
            &pipeline_layout,
            subpass,
        );
        
        pipeline_desc
            .blender
            .targets
            .push(ColorBlendDesc(ColorMask::ALL, BlendState::ALPHA));

        pipeline_desc.vertex_buffers.push(VertexBufferDesc {
            binding: 0,
            stride: std::mem::size_of::<Vertex>() as u32,
            rate: 0,
        });

        pipeline_desc.attributes.push(AttributeDesc {
            location: 0,
            binding: 0,
            element: Element {
                format: Format::Rgb32Float,
                offset: 0,
            },
        });

        pipeline_desc.attributes.push(AttributeDesc {
            location: 1,
            binding: 0,
            element: Element {
                format: Format::Rgba32Float,
                offset: 12,
            },
        });

        device
            .create_graphics_pipeline(&pipeline_desc, None)
            .unwrap()
    };

    let memory_types = physical_device.memory_properties().memory_types;
    
    let teapot = utils::load_obj(String::from("../assets/models/teapot.obj"));
    let mesh = if std::env::args().nth(1) == Some("teapot".into()) {
        &teapot
    } else {
        MESH
    };

    //println!("{:?}", mesh);

    let (vertex_buffer, vertex_buffer_memory) = {
        let item_count = mesh.len();
        let stride = std::mem::size_of::<Vertex>() as u64;
        let buffer_len = item_count as u64 * stride;
        let unbound_buffer = device
            .create_buffer(buffer_len, buffer::Usage::VERTEX)
            .unwrap();

        let req = device.get_buffer_requirements(&unbound_buffer);

        let upload_type = memory_types
            .iter()
            .enumerate()
            .find(|(id, ty)| {
                let type_supported = req.type_mask & (1_u64 << id) != 0;
                type_supported && ty.properties.contains(Properties::CPU_VISIBLE)
            }).map(|(id, _ty)| MemoryTypeId(id))
            .expect("Could not find approprate vertex buffer memory type.");

        let buffer_memory = device.allocate_memory(upload_type, req.size).unwrap();
        let buffer = device
            .bind_buffer_memory(&buffer_memory, 0, unbound_buffer)
            .unwrap();
        
        {
            let mut dest = device
                .acquire_mapping_writer::<Vertex>(&buffer_memory, 0..buffer_len)
                .unwrap();
            dest.copy_from_slice(mesh);
            device.release_mapping_writer(dest);
        }

        (buffer, buffer_memory)
    };

    let frame_semaphore = device.create_semaphore().unwrap();
    let present_semaphore = device.create_semaphore().unwrap();

    let mut rebuild_swapchain = false;

    //add the block sprite to our cache
    //let block = String::from("../../assets/block.BMP");
    //graphics::load_sprites(block, &texture_creator, &mut texcache);
    //let background = String::from("../../assets/tet.BMP");
    //graphics::load_sprites(background, &texture_creator, &mut texcache);

    let mut starting_time: Instant = Instant::now();
    let mut pos = 40;

    let mut i = 1;
    let mut swapchain_stuff: Option<(_, _, _, _)> = None;

    events_loop.run_forever(|event| {

        let mut quitting = false;

        match event {
            winit::Event::WindowEvent { event, .. } => match event {                
                winit::WindowEvent::KeyboardInput {
                    input:
                        winit::KeyboardInput {
                            virtual_keycode: Some(winit::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                } => quitting = true, |
                CloseRequested => { quitting = true },
                winit::WindowEvent::Resized(_) => { rebuild_swapchain = true; }
                _ => (),
            },
            _ => (),
        }

        if (rebuild_swapchain || quitting) && swapchain_stuff.is_some() {
            let (swapchain, _extent, frame_views, framebuffers) = swapchain_stuff.take().unwrap();
            device.wait_idle().unwrap();
            command_pool.reset();
            for framebuffer in framebuffers {
                device.destroy_framebuffer(framebuffer);
            }

            for image_view in frame_views {
                device.destroy_image_view(image_view);
            }
            device.destroy_swapchain(swapchain);
        }

        if quitting {
            return winit::ControlFlow::Break;
        }

        if swapchain_stuff.is_none() {
            rebuild_swapchain = false;

            let (caps, _, _) = surface.compatibility(physical_device);
            let swap_config = SwapchainConfig::from_caps(&caps, surface_color_format);
            let extent = swap_config.extent.to_extent();
            let (swapchain, backbuffer) = device.create_swapchain(&mut surface, swap_config, None).unwrap();

            let (frame_views, framebuffers) = match backbuffer {
                Backbuffer::Images(images) => {
                    let color_range = SubresourceRange {
                        aspects: Aspects::COLOR,
                        levels: 0..1,
                        layers: 0..1,
                    };

                    let image_views = images
                        .iter()
                        .map(|image| {
                            device
                                .create_image_view(
                                    image,
                                    ViewKind::D2,
                                    surface_color_format,
                                    Swizzle::NO,
                                    color_range.clone(),
                                ).unwrap()
                        }).collect::<Vec<_>>();

                    let fbos = image_views
                        .iter()
                        .map(|image_view| {
                            device
                                .create_framebuffer(&render_pass, vec![image_view], extent)
                                .unwrap()
                        }).collect();

                    (image_views, fbos)
                }
                Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
            };

            // Store the new stuff.
            swapchain_stuff = Some((swapchain, extent, frame_views, framebuffers));
        }

        let (swapchain, extent, _frame_views, framebuffers) = swapchain_stuff.as_mut().unwrap();

        let loop_time: Instant = Instant::now();
        i = (i + 1) % 255;

        let ending_time: Duration = Instant::now().duration_since(loop_time);
        let delta_time: Duration = Duration::from_millis(250);

        if starting_time.elapsed() > delta_time {
            starting_time = Instant::now();
            if pos <= 466 {pos = pos + 26;}
            //graphics::update(&mut canvas, &mut texcache, i as u8, pos);
        }

        match (Duration::from_millis(1000) / FPS).checked_sub(ending_time) {
            Some(i) => ::std::thread::sleep(i),
            _ => (),
        };

        command_pool.reset();

        // A swapchain contains multiple images - which one should we draw on? This
        // returns the index of the image we'll use. The image may not be ready for
        // rendering yet, but will signal frame_semaphore when it is.
        let frame_index: SwapImageIndex = swapchain
            .acquire_image(!0, FrameSync::Semaphore(&frame_semaphore))
            .expect("Failed to acquire frame");

        // We have to build a command buffer before we send it off to draw.
        // We don't technically have to do this every frame, but if it needs to
        // change every frame, then we do.
        let finished_command_buffer = {
            let mut command_buffer = command_pool.acquire_command_buffer(false);

            // Define a rectangle on screen to draw into.
            // In this case, the whole screen.
            let viewport = Viewport {
                rect: Rect {
                    x: 0,
                    y: 0,
                    w: extent.width as i16,
                    h: extent.height as i16,
                },
                depth: 0.0..1.0,
            };

            command_buffer.set_viewports(0, &[viewport.clone()]);
            command_buffer.set_scissors(0, &[viewport.rect]);

            // Choose a pipeline to use.
            command_buffer.bind_graphics_pipeline(&pipeline);
            command_buffer.bind_vertex_buffers(0, vec![(&vertex_buffer, 0)]);

            {
                // Clear the screen and begin the render pass.
                let mut encoder = command_buffer.begin_render_pass_inline(
                    &render_pass,
                    &framebuffers[frame_index as usize],
                    viewport.rect,
                    &[ClearValue::Color(ClearColor::Float([0.0, 0.0, 0.0, 1.0]))],
                );

                let num_vertices = mesh.len() as u32;
                encoder.draw(0..num_vertices, 0..1);
            }

            // Finish building the command buffer - it's now ready to send to the
            // GPU.
            command_buffer.finish()
        };

        // This is what we submit to the command queue. We wait until frame_semaphore
        // is signalled, at which point we know our chosen image is available to draw
        // on.
        let submission = Submission::new()
            .wait_on(&[(&frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)])
            .signal(&[&present_semaphore])
            .submit(vec![finished_command_buffer]);

        // We submit the submission to one of our command queues, which will signal
        // frame_fence once rendering is completed.
        queue_group.queues[0].submit(submission, None);

        // We first wait for the rendering to complete...
        // TODO: Fix up for semaphores

        // ...and then present the image on screen!
        swapchain
            .present(
                &mut queue_group.queues[0],
                frame_index,
                vec![&present_semaphore],
            ).expect("Present failed");

        winit::ControlFlow::Continue

            //debug
            //println!("{:#?}", Instant::now().duration_since(starting_time));
    });

    // Cleanup
    device.destroy_graphics_pipeline(pipeline);
    device.destroy_pipeline_layout(pipeline_layout);
    device.destroy_render_pass(render_pass);
    device.destroy_shader_module(vertex_shader_module);
    device.destroy_shader_module(fragment_shader_module);
    device.destroy_command_pool(command_pool.into_raw());
    device.destroy_buffer(vertex_buffer);
    device.free_memory(vertex_buffer_memory);
    device.destroy_semaphore(frame_semaphore);
    device.destroy_semaphore(present_semaphore);
}
