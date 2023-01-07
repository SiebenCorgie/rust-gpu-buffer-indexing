use marpii::{context::Ctx, ash::vk};
use marpii_rmg::{Rmg, BufferHandle, Task};
use marpii_rmg_tasks::UploadBuffer;
use shared::BufTyOne;


const SHADER_COMP: &[u8] = include_bytes!("../resources/shadercrate.spv");


struct CopyTask{
    src: BufferHandle<shared::BufTyOne>,
    dst: BufferHandle<shared::BufTyTwo>,
}

impl CopyTask{

    const SRC_DTA: [BufTyOne; 4] = [
        BufTyOne{
            a: 1.0,
            b: 2.0,
            pad: [0.0; 2]
        },
        BufTyOne{
            a: 3.0,
            b: 3.0,
            pad: [0.0; 2]
        },
        BufTyOne{
            a: 4.0,
            b: 5.0,
            pad: [0.0; 2]
        },
        BufTyOne{
            a: 6.0,
            b: 6.0,
            pad: [0.0; 2]
        }
    ];

    fn new(rmg: &mut Rmg) -> Self{

        let upload = UploadBuffer::new(rmg, &Self::SRC_DTA).unwrap();
        //upload
        rmg.record()
            .add_task(&mut upload)
            .unwrap()
            .execute().unwrap();
        //create dst buffer
        let dst = rmg.new_buffer(Self::SRC_DTA.len(), None).unwrap();

        //load the holy shader
        let shader_module = ShaderModule::new_from_bytes(&rmg.ctx.device, SHADER_COMP)?;
        let shader_stage = shader_module.into_shader_stage(vk::ShaderStageFlags::COMPUTE, "main");
        //No additional descriptors for us
        let layout = rmg.resources().bindless_layout();
        let pipeline = Arc::new(ComputePipeline::new(
            &rmg.ctx.device,
            &shader_stage,
            None,
            layout,
        )?);


        CopyTask { src: upload.buffer, dst }
    }
}

impl Task for CopyTask{
    fn name(&self) -> &'static str {
        "TestCopy"
    }

    fn queue_flags(&self) -> marpii::ash::vk::QueueFlags {
        vk::QueueFlags::COMPUTE
    }

    fn register(&self, registry: &mut marpii_rmg::ResourceRegistry) {
        registry.request_buffer(&self.src, vk::PipelineStageFlags2::COMPUTE_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ);
        registry.request_buffer(&self.dst, vk::PipelineStageFlags2::COMPUTE_SHADER, vk::AccessFlags2::SHADER_STORAGE_WRITE);
    }

    fn record(
        &mut self,
        device: &std::sync::Arc<marpii::context::Device>,
        command_buffer: &vk::CommandBuffer,
        resources: &marpii_rmg::Resources,
    ) {

    }
}

fn main() -> Result<(), anyhow::Error> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();
    let context = Ctx::new_headless(true)?;
    let mut rmg = Rmg::new(context)?;





    Ok(())
}
