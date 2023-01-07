use marpii::{context::Ctx, ash::vk, resources::{ShaderModule, ComputePipeline, PushConstant}};
use marpii_rmg::{Rmg, BufferHandle, Task};
use marpii_rmg_task_shared::ResourceHandle;
use marpii_rmg_tasks::UploadBuffer;
use shared::BufTyOne;
use std::sync::Arc;

const SHADER_COMP: &[u8] = include_bytes!("../../resources/shadercrate.spv");


struct CopyTask{
    src: BufferHandle<shared::BufTyOne>,
    dst: BufferHandle<shared::BufTyTwo>,
    push: PushConstant<shared::Push>,
    pipeline: Arc<ComputePipeline>,
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
    const SUBGROUP_COUNT: u32 = 64;
    fn new(rmg: &mut Rmg) -> Self{

        let mut upload = UploadBuffer::new(rmg, &Self::SRC_DTA).unwrap();
        //upload
        rmg.record()
            .add_task(&mut upload)
            .unwrap()
            .execute().unwrap();
        //create dst buffer
        let dst = rmg.new_buffer(Self::SRC_DTA.len(), None).unwrap();

        //load the holy shader
        let shader_module = ShaderModule::new_from_bytes(&rmg.ctx.device, SHADER_COMP).unwrap();
        let shader_stage = shader_module.into_shader_stage(vk::ShaderStageFlags::COMPUTE, "main");
        //No additional descriptors for us
        let layout = rmg.resources().bindless_layout();
        let pipeline = Arc::new(ComputePipeline::new(
            &rmg.ctx.device,
            &shader_stage,
            None,
            layout,
        ).unwrap());

        let push = PushConstant::new(
            shared::Push{
                src_hdl: ResourceHandle::INVALID,
                dst_hdl: ResourceHandle::INVALID,
                size: Self::SRC_DTA.len() as u32,
                pad: 0
            },
            vk::ShaderStageFlags::COMPUTE
        );

        CopyTask { src: upload.buffer, dst, push, pipeline }
    }


    fn dispatch_count() -> u32 {
        ((Self::SRC_DTA.len() as f32) / Self::SUBGROUP_COUNT as f32).ceil() as u32
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
        registry.request_buffer(&self.src, vk::PipelineStageFlags2::COMPUTE_SHADER, vk::AccessFlags2::SHADER_STORAGE_READ).unwrap();
        registry.request_buffer(&self.dst, vk::PipelineStageFlags2::COMPUTE_SHADER, vk::AccessFlags2::SHADER_STORAGE_WRITE).unwrap();
        registry.register_asset(self.pipeline.clone());
    }

    fn pre_record(&mut self, resources: &mut marpii_rmg::Resources, _ctx: &marpii_rmg::CtxRmg) -> Result<(), marpii_rmg::RecordError> {
        self.push.get_content_mut().src_hdl = resources.resource_handle_or_bind(&self.src)?;
        self.push.get_content_mut().dst_hdl = resources.resource_handle_or_bind(&self.dst)?;
        Ok(())
    }

    fn record(
        &mut self,
        device: &std::sync::Arc<marpii::context::Device>,
        command_buffer: &vk::CommandBuffer,
        resources: &marpii_rmg::Resources,
    ) {
        //bind, setup push constant and execute
        unsafe {
            device.inner.cmd_bind_pipeline(
                *command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline.pipeline,
            );
            device.inner.cmd_push_constants(
                *command_buffer,
                self.pipeline.layout.layout,
                vk::ShaderStageFlags::ALL,
                0,
                self.push.content_as_bytes(),
            );

            device
                .inner
                .cmd_dispatch(*command_buffer, Self::dispatch_count(), 1, 1);
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Warn)
        .init()
        .unwrap();
    let context = Ctx::new_default_headless(true)?;
    let mut rmg = Rmg::new(context)?;
    let mut task = CopyTask::new(&mut rmg);

    rmg.record()
        .add_task(&mut task)
        .unwrap()
        .execute()
        .unwrap();


    Ok(())
}
