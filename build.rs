fn main()
{
    let bindings = bindgen::Builder::default()

        .header("gensudoku/sudoku.h")

        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))

        .generate()

        .expect("Unable to generate bindings");
		
		bindings
        .write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}