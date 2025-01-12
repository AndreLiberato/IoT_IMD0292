mod app;
mod entry;

fn main() -> eframe::Result {
    entry::nonwasm::main()
}
