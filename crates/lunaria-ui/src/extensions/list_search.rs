use gpui_kit::{App, Context, Entity, Task};

pub trait ListSearchExt<V: 'static> {
    fn perform_search(
        &self,
        query: &str,
        cx: &mut App,
        set_query: fn(&mut V, &str, &mut Context<V>),
    ) -> Task<()>;
}

impl<V: 'static> ListSearchExt<V> for Entity<V> {
    fn perform_search(
        &self,
        query: &str,
        cx: &mut App,
        set_query: fn(&mut V, &str, &mut Context<V>),
    ) -> Task<()> {
        self.update(cx, |view_model, cx| {
            set_query(view_model, query, cx);
        });

        Task::ready(())
    }
}
