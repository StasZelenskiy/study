use wee_alloc::WeeAlloc;

#[global_allocator]
pub static ALLOC: WeeAlloc = WeeAlloc::INIT;
