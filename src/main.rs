

use std::ffi::CString;
use std::os::raw::c_char;
use libloading::{Library, Symbol};





fn main() {
   
    unsafe {
        // Carga la biblioteca DLL
        let library = Library::new("CMakeProject2.dll").expect("Error al cargar la biblioteca DLL");

        // Obtiene los símbolos de las funciones desde la biblioteca
         let print_pdf_file: Symbol<unsafe extern "C" fn(*const c_char, *const c_char, *const c_char, *const c_char) -> i32> =
             library.get(b"printPdfFile").expect("Error al obtener el símbolo de la función printPdfFile");
        let get_printer_list: Symbol<unsafe extern "C" fn() -> *const c_char> =
            library.get(b"getPrinterList").expect("Error al obtener el símbolo de la función getPrinterList");

        // Llama a la función getPrinterList cargada
        let printer_list_ptr = get_printer_list();
        let printer_list = CString::from_raw(printer_list_ptr as *mut c_char).into_string().unwrap();
        println!("Lista de impresoras: {}", printer_list);

        // Llama a la función printPdfFile cargada
        let fileprint = CString::new("C:/Users/51925/Downloads/10744490630-09-T001-3.pdf").expect("Error al crear CString");
        let printer_name = CString::new("Microsoft Print to PDF").expect("Error al crear CString");
        let page_number = CString::new("0").expect("Error al crear CString");
        let orientation = CString::new("PORTRAIT").expect("Error al crear CString");
        let result = print_pdf_file(fileprint.as_ptr(), printer_name.as_ptr(), page_number.as_ptr(), orientation.as_ptr());
        println!("Resultado: {}", result);
    }
}

// main.rs
// use libloader::{libloading, get_libfn};
// fn main() {
//     get_libfn!("CMakeProject2.dll", "getPrinterList", my_println, ());
//     my_println();

//     // get_libfn!("libstd.dylib", "add", my_add, usize, a: usize, b: usize);
//     // println!("10 + 20 = {}", my_add(10, 20));

//     // get_libfn!("libstd.dylib", "print_hello", my_print_hello, ());
//     // my_print_hello();
// }