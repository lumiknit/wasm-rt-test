use wasmer::{imports, Function, Instance, Module, Store, Value};

fn wasm_print(value: i64) {
    println!("Print: {}", value);
}

fn main() {
    let src = std::include_str!("../../example.wat");

    let mut store = Store::default();
    let module = Module::new(&store, &src).map_err(|e| eprintln!("Error: {}", e)).unwrap();

    let import_object = imports! {
        "runtime" => {
            "print" => Function::new_typed(&mut store, wasm_print),
        }
    };
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let fibo = instance.exports.get_function("print_fibo").unwrap();
    let input = Value::I64(30);
    let result = fibo.call(&mut store, &[input.clone()]);

    match result {
        Ok(_) => println!("Fibo({}) = {:?}", input.i64().unwrap(), result.unwrap()),
        Err(e) => println!("Error: {:?}", e),
    }
}
