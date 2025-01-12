pub mod painel;

pub use painel::Painel;

// pub trait App {
//     fn new(cc: &eframe::CreationContext) -> Self;
// }

pub trait App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self;
}
