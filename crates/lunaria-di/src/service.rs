// #[service] 为具体的服务生成 build trait 实现
pub trait BuildService<C>: Sized {
    fn build(cx: &C) -> anyhow::Result<Self>;
}
