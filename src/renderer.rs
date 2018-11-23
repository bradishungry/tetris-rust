//use std::collections::hash_map::HashMap;
use std::time::{Duration, Instant};

use winit;
use winit::WindowEvent::*;

//hal
use back::{Backend, self};
use hal::{
    buffer,
    command::{BufferImageCopy, ClearColor, ClearDepthStencil, ClearValue},
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{
        self as img, Access, Extent, Filter, Layout, Offset,
        SubresourceLayers, SubresourceRange, ViewCapabilities, ViewKind, WrapMode,
    },
    memory::{Barrier, Dependencies, Properties},
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    },
    pool::CommandPoolCreateFlags,
    pso::{
        AttributeDesc, BlendState, ColorBlendDesc, ColorMask, Comparison, DepthTest, DepthStencilDesc, Descriptor, 
        DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorSetWrite, DescriptorType, 
        Element, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, ShaderStageFlags, StencilTest, VertexBufferDesc, 
        Viewport,
    },
    queue::Submission,
    Backbuffer, Device, DescriptorPool, FrameSync, Graphics, Instance, PhysicalDevice, Primitive, Surface, SwapImageIndex,
    Swapchain, SwapchainConfig,
};

use utils::{self, PushConstants, UniformBlock, Vertex};

const MESH: &[Vertex] = &[
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

pub fn render_loop() {

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
    let adapter = instance.enumerate_adapters().remove(0);
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
    
    let depth_format = Format::D32FloatS8Uint;
    
    let render_pass = {
        let color_attachment = Attachment {
            format: Some(surface_color_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::Present,
        };

        let depth_attachment = Attachment {
            format: Some(depth_format),
            samples: 1,
            ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::DontCare),
            stencil_ops: AttachmentOps::DONT_CARE,
            layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal,
        };

        let subpass = SubpassDesc {
            colors: &[(0, Layout::ColorAttachmentOptimal)],
            depth_stencil: Some(&(1, Layout::DepthStencilAttachmentOptimal)),
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

        device.create_render_pass(
            &[color_attachment, depth_attachment],
            &[subpass],
            &[dependency],
        ).unwrap()
    };

    let set_layout = device.create_descriptor_set_layout(
        &[
        DescriptorSetLayoutBinding {
            binding: 0,
            ty: DescriptorType::UniformBuffer,
            count: 1,
            stage_flags: ShaderStageFlags::VERTEX,
            immutable_samplers: false,
        },
        DescriptorSetLayoutBinding {
            binding: 1,
            ty: DescriptorType::SampledImage,
            count: 1,
            stage_flags: ShaderStageFlags::FRAGMENT,
            immutable_samplers: false,
        },
        DescriptorSetLayoutBinding {
            binding: 2,
            ty: DescriptorType::Sampler,
            count: 1,
            stage_flags: ShaderStageFlags::FRAGMENT,
            immutable_samplers: false,
        },
        ],
        &[],
        ).unwrap();

    let num_push_constants = {
        let size_in_bytes = std::mem::size_of::<PushConstants>();
        let size_of_push_constant = std::mem::size_of::<u32>();
        size_in_bytes / size_of_push_constant
    };
    

    let pipeline_layout = device.create_pipeline_layout(vec![&set_layout], &[(ShaderStageFlags::VERTEX, 0..(num_push_constants as u32))], ).unwrap();


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

        pipeline_desc.attributes.push(AttributeDesc {
            location: 2,
            binding: 0,
            element: Element {
                format: Format::Rgba32Float,
                offset: 28,
            },
        });

        pipeline_desc.depth_stencil = DepthStencilDesc {
            depth: DepthTest::On {
                fun: Comparison::Less,
                write: true,
            },
            depth_bounds: false,
            stencil: StencilTest::default(),
        };

        device
            .create_graphics_pipeline(&pipeline_desc, None)
            .unwrap()
    };

    let mut desc_pool = device.create_descriptor_pool(
        1,
        &[
            DescriptorRangeDesc {
                ty: DescriptorType::UniformBuffer,
                count: 1,
            },
            DescriptorRangeDesc {
                ty: DescriptorType::SampledImage,
                count: 1,
            },
            DescriptorRangeDesc {
                ty: DescriptorType::Sampler,
                count: 1,
            },
        ],
    ).unwrap();

    let desc_set = desc_pool.allocate_set(&set_layout).unwrap();

    let memory_types = physical_device.memory_properties().memory_types;
    
    let teapot = utils::load_obj(String::from("../assets/models/teapot.obj"));
    let mesh = if std::env::args().nth(1) == Some("teapot".into()) {
        &teapot
    } else {
        MESH
    };

    let (vertex_buffer, vertex_buffer_memory) = utils::create_buffer::<Backend, Vertex>(
        &device,
        &memory_types,
        Properties::CPU_VISIBLE,
        buffer::Usage::VERTEX,
        &mesh,
    );

    let (uniform_buffer, mut uniform_memory) = utils::create_buffer::<Backend, UniformBlock>(
        &device,
        &memory_types,
        Properties::CPU_VISIBLE,
        buffer::Usage::UNIFORM,
        &[UniformBlock {
            projection: Default::default(),
        }],
    );
    
    let texture_fence = device.create_fence(false).unwrap();
    
    let (texture_image, texture_memory, texture_view, texture_sampler) = {
        let image_bytes = include_bytes!("../assets/block.png");
        let img = image::load_from_memory(image_bytes.as_ref())
            .expect("Failed to load image.")
            .to_rgba();
        let (width, height) = img.dimensions();

        let (texture_image, texture_memory, texture_view) = utils::create_image::<Backend>(
            &device,
            &memory_types,
            width,
            height,
            Format::Rgba8Srgb,
            img::Usage::TRANSFER_DST | img::Usage::SAMPLED,
            Aspects::COLOR,
        );

        let texture_sampler =
            device.create_sampler(img::SamplerInfo::new(Filter::Linear, WrapMode::Clamp)).unwrap();

        {
            let row_alignment_mask =
                physical_device.limits().min_buffer_copy_pitch_alignment as u32 - 1;
            let image_stride = 4usize;
            let row_pitch =
                (width * image_stride as u32 + row_alignment_mask) & !row_alignment_mask;
            let upload_size = u64::from(height * row_pitch);

            let (image_upload_buffer, mut image_upload_memory) = utils::empty_buffer::<Backend, u8>(
                &device,
                &memory_types,
                Properties::CPU_VISIBLE,
                buffer::Usage::TRANSFER_SRC,
                upload_size as usize,
            );

            {
                let mut data = device
                    .acquire_mapping_writer::<u8>(&image_upload_memory, 0..upload_size)
                    .unwrap();

                for y in 0..height as usize {
                    let row = &(*img)[y * (width as usize) * image_stride
                                          ..(y + 1) * (width as usize) * image_stride];
                    let dest_base = y * row_pitch as usize;
                    data[dest_base..dest_base + row.len()].copy_from_slice(row);
                }

                device.release_mapping_writer(data);
            }

            let submit = {
                let mut cmd_buffer = command_pool.acquire_command_buffer(false);

                let image_barrier = Barrier::Image {
                    states: (Access::empty(), Layout::Undefined)
                        ..(Access::TRANSFER_WRITE, Layout::TransferDstOptimal),
                    target: &texture_image,
                    range: SubresourceRange {
                        aspects: Aspects::COLOR,
                        levels: 0..1,
                        layers: 0..1,
                    },
                };

                cmd_buffer.pipeline_barrier(
                    PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
                    Dependencies::empty(),
                    &[image_barrier],
                );

                cmd_buffer.copy_buffer_to_image(
                    &image_upload_buffer,
                    &texture_image,
                    Layout::TransferDstOptimal,
                    &[BufferImageCopy {
                        buffer_offset: 0,
                        buffer_width: row_pitch / (image_stride as u32),
                        buffer_height: height as u32,
                        image_layers: SubresourceLayers {
                            aspects: Aspects::COLOR,
                            level: 0,
                            layers: 0..1,
                        },
                        image_offset: Offset { x: 0, y: 0, z: 0 },
                        image_extent: Extent {
                            width,
                            height,
                            depth: 1,
                        },
                    }],
                );

                let image_barrier = Barrier::Image {
                    states: (Access::TRANSFER_WRITE, Layout::TransferDstOptimal)
                        ..(Access::SHADER_READ, Layout::ShaderReadOnlyOptimal),
                    target: &texture_image,
                    range: SubresourceRange {
                        aspects: Aspects::COLOR,
                        levels: 0..1,
                        layers: 0..1,
                    },
                };

                cmd_buffer.pipeline_barrier(
                    PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
                    Dependencies::empty(),
                    &[image_barrier],
                );

                cmd_buffer.finish()
            };

            let submission = Submission::new().submit(Some(submit));
            queue_group.queues[0].submit(submission, Some(&texture_fence));

            // Cleanup staging resources
            device.destroy_buffer(image_upload_buffer);
            device.free_memory(image_upload_memory);
        }

        (texture_image, texture_memory, texture_view, texture_sampler)
    };

    device.write_descriptor_sets(vec![
        DescriptorSetWrite {
            set: &desc_set,
            binding: 0,
            array_offset: 0,
            descriptors: Some(Descriptor::Buffer(&uniform_buffer, None..None)),
        },

        DescriptorSetWrite {
            set: &desc_set,
            binding: 1,
            array_offset: 0,
            descriptors: Some(Descriptor::Image(&texture_view, Layout::Undefined)),
        },

        DescriptorSetWrite {
            set: &desc_set,
            binding: 2,
            array_offset: 0,
            descriptors: Some(Descriptor::Sampler(&texture_sampler)),
        },
    ]);

    let diamonds = vec![
        PushConstants {
            tint: [0.6, 0.6, 0.6, 1.0],
            position: [-0.1, 0.0, 0.3],
        },
        PushConstants {
            tint: [0.2, 0.2, 0.2, 1.0],
            position: [0.3, 0.8, 0.5],
        },
        PushConstants {
            tint: [0.4, 0.4, 0.4, 1.0],
            position: [0.1, 0.4, 0.4],
        },
        PushConstants {
            tint: [1.0, 1.0, 1.0, 1.0],
            position: [-0.5, -0.8, 0.1],
        },
        PushConstants {
            tint: [0.8, 0.8, 0.8, 1.0],
            position: [-0.3, -0.4, 0.2],
        },
    ];

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

    let mut swapchain_stuff: Option<(_, _, _, _, _, _, _)> = None;

    device.wait_for_fence(&texture_fence, !0);
    
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
            let (
                swapchain,
                _extent,
                frame_views,
                framebuffers,
                depth_image,
                depth_image_view,
                depth_image_memory,
            ) = swapchain_stuff.take().unwrap();           

            device.wait_idle().unwrap();
            command_pool.reset();
            for framebuffer in framebuffers {
                device.destroy_framebuffer(framebuffer);
            }

            for image_view in frame_views {
                device.destroy_image_view(image_view);
            }

            device.destroy_image_view(depth_image_view);
            device.destroy_image(depth_image);
            device.free_memory(depth_image_memory);

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

            let (depth_image, depth_image_memory, depth_image_view) = {
                let kind =
                    img::Kind::D2(extent.width as img::Size, extent.height as img::Size, 1, 1);

                let unbound_depth_image = device
                    .create_image(
                        kind,
                        1,
                        depth_format,
                        img::Tiling::Optimal,
                        img::Usage::DEPTH_STENCIL_ATTACHMENT,
                        ViewCapabilities::empty(),
                    ).expect("Failed to create unbound depth image");

                let image_req = device.get_image_requirements(&unbound_depth_image);

                let device_type = memory_types
                    .iter()
                    .enumerate()
                    .position(|(id, memory_type)| {
                        image_req.type_mask & (1 << id) != 0
                            && memory_type.properties.contains(Properties::DEVICE_LOCAL)
                    }).unwrap()
                    .into();

                let depth_image_memory = device
                    .allocate_memory(device_type, image_req.size)
                    .expect("Failed to allocate depth image");

                let depth_image = device
                    .bind_image_memory(&depth_image_memory, 0, unbound_depth_image)
                    .expect("Failed to bind depth image");

                let depth_image_view = device
                    .create_image_view(
                        &depth_image,
                        img::ViewKind::D2,
                        depth_format,
                        Swizzle::NO,
                        img::SubresourceRange {
                            aspects: Aspects::DEPTH | Aspects::STENCIL,
                            levels: 0..1,
                            layers: 0..1,
                        },
                    ).expect("Failed to create image view");

                (depth_image, depth_image_memory, depth_image_view)
            };

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
                                .create_framebuffer(&render_pass, 
                                                vec![image_view, &depth_image_view],
                                                extent,
                                                ).unwrap()
                        }).collect();

                    (image_views, fbos)
                }
                Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
            };

            // Store the new stuff.
            swapchain_stuff = Some((
                swapchain,
                extent,
                frame_views,
                framebuffers,
                depth_image,
                depth_image_view,
                depth_image_memory,
            ));
        }

        let (
            swapchain,
            extent,
            _frame_views,
            framebuffers,
            _depth_image,
            _depth_image_view,
            _depth_image_memory,
        ) = swapchain_stuff.as_mut().unwrap();

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
        
        let (width, height) = (extent.width, extent.height);
        let aspect_corrected_x = height as f32 / width as f32;
        let zoom = (delta_time.as_secs() as f32).cos() * 0.33 + 0.67;
        let x_scale = aspect_corrected_x * zoom;
        let y_scale = zoom;

        utils::fill_buffer::<Backend, UniformBlock>(
            &device,
            &mut uniform_memory,
            &[UniformBlock {
                projection: [
                    [x_scale, 0.0, 0.0, 0.0],
                    [0.0, y_scale, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0],
                ],
            }],
        );

        command_pool.reset();

        let frame_index: SwapImageIndex = {
            match swapchain.acquire_image(!0, FrameSync::Semaphore(&frame_semaphore)) {
                Ok(i) => i,
                Err(_) => {
                    rebuild_swapchain = true;
                    0 //TODO: FIX THIS
                }
            }
        };
        
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

            command_buffer.bind_graphics_descriptor_sets(&pipeline_layout, 0, vec![&desc_set], &[]);

            {
                let mut encoder = command_buffer.begin_render_pass_inline(
                    &render_pass,
                    &framebuffers[frame_index as usize],
                    viewport.rect,
                    &[
                        ClearValue::Color(ClearColor::Float([0.0, 0.0, 0.0, 1.0])),
                        ClearValue::DepthStencil(ClearDepthStencil(1.0, 0)),
                    ],
                );

                let num_vertices = mesh.len() as u32;
                
                for diamond in &diamonds {
                    let push_constants = {
                        let start_ptr = diamond as *const PushConstants as *const u32;
                        unsafe { std::slice::from_raw_parts(start_ptr, num_push_constants) }
                    };
                    
                    encoder.push_graphics_constants(
                        &pipeline_layout,
                        ShaderStageFlags::VERTEX,
                        0,
                        push_constants,
                        );
             
                    encoder.draw(0..num_vertices, 0..1);
                }
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
        let result = swapchain.present(
            &mut queue_group.queues[0],
            frame_index,
            vec![&present_semaphore],
        );

        if result.is_err() {
            rebuild_swapchain = true;
        }

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
    device.destroy_buffer(uniform_buffer);
    device.free_memory(uniform_memory);
    device.destroy_buffer(vertex_buffer);
    device.free_memory(vertex_buffer_memory);
    device.destroy_image(texture_image);
    device.destroy_image_view(texture_view);
    device.destroy_sampler(texture_sampler);
    device.free_memory(texture_memory);
    device.destroy_semaphore(frame_semaphore);
    device.destroy_semaphore(present_semaphore);
}
