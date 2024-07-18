extern crate winapi;
extern crate winres;
use winres::WindowsResource;
fn main() {
    WindowsResource::new()
        .set_icon("icon.ico")
        .set_language(winapi::um::winnt::MAKELANGID(
            winapi::um::winnt::LANG_PORTUGUESE,
            winapi::um::winnt::SUBLANG_PORTUGUESE_BRAZILIAN,
        ))
        .set("ProductName", "Gravis CGBR")
        .set(
            "FileDescription",
            "Gravis CGBR - Sistemas de Gestao Comercial ERP - CNPJ: 10.868.122/0001-58",
        )
        .set("LegalCopyright", "Gravis CGBR")
        .set("Legaltrademarks", "Gravis Tec")
        .compile()
        .unwrap();
}
