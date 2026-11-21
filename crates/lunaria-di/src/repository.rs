use anyhow::{Result, anyhow, ensure};
use std::any::{Any, type_name};
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SourceKind {
    Local,
}

// 根据当前项目的数据源获取底层 database 类型
pub trait RepositoryResolver {
    fn current_repository<T: 'static>(&self) -> Result<T>;
}

pub type Backend = Arc<dyn Any + Send + Sync>;

pub fn backend_as<T: Any + Send + Sync>(backend: &Backend) -> Result<Arc<T>> {
    backend
        .clone()
        .downcast::<T>()
        .map_err(|_| anyhow!("Repository requires backend {}", type_name::<T>()))
}

pub struct RepositoryFactory {
    pub key: SourceKind,
    pub factory: &'static (dyn Any + Send + Sync),
}

inventory::collect!(RepositoryFactory);

pub fn resolve_repository<T: 'static>(key: SourceKind, backend: &Backend) -> Result<T> {
    // 按数据源和 repository 接口筛选 factory
    let mut factories = inventory::iter::<RepositoryFactory>
        .into_iter()
        .filter(|entry| entry.key == key)
        .filter_map(|entry| entry.factory.downcast_ref::<fn(&Backend) -> Result<T>>());

    let name = type_name::<T>();
    // 取出第一个匹配的 factory
    let factory = factories
        .next()
        .ok_or_else(|| anyhow!("No repository registered for {name} with key {key:?}"))?;

    ensure!(
        factories.next().is_none(),
        "Multiple repositories registered for {name} with key {key:?}"
    );

    //
    factory(backend)
}
