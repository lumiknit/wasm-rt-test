use wasmtime::{Engine, Func, Instance, Module, Store};

fn wasm_print(value: i64) {
    println!("Print: {}", value);
}

fn main() {
    // Load .wat file example.wat
    let src = std::include_str!("../../example.wat");

    // Convert the .wat file to a module
    let wasm = wat::parse_str(src).unwrap();

    let engine = Engine::default();
    let module = Module::from_binary(&engine, wasm.as_slice()).unwrap();
    let mut store = Store::new(&engine, ());

    let fn_print = Func::wrap(&mut store, wasm_print as fn(i64));

    let instance = Instance::new(&mut store, &module, &[
        fn_print.into(),
    ]).unwrap();


    let fibo = instance
        .get_func(&mut store, "print_fibo").unwrap()
        .typed::<i64, ()>(&store).unwrap();
    let input = 30i64;
    let result = fibo.call(store, input).unwrap();
    println!("Fibo({}) = {:?}", input, result);
}
