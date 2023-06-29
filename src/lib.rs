pub mod lib {
    use cpython::{GILGuard, Python, ObjectProtocol, ToPyObject};

    pub fn copy_to_clipboard(text: &str) {
        let gil: GILGuard = Python::acquire_gil();
        let py = gil.python();
        let pyperclip = py.import("pyperclip").unwrap();
        let copy_fn = pyperclip.get(py, "copy").unwrap();

        let text_pyobj = text.to_py_object(py);
        let args = (text_pyobj,);
        copy_fn.call(py, args, None).unwrap();
    }
}


