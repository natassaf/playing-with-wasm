
// use core_affinity::{get_core_ids, set_for_current, CoreId};
// use std::{thread, time::Duration};
// use wasmtime::{Engine, Module, Store, Instance, TypedFunc};
// use tokio::{self, task};


// #[tokio::main]
// async fn main() {
//     // Set up Wasmtime engine and module outside blocking
//     let engine = Engine::default();
//     let core_ids = get_core_ids().expect("Failed to get core IDs");
//     let core6: CoreId = core_ids[7];
//     let path_to_module: &'static str = "/Users/athanasiapharmake/workspace/fib-host/wasm-modules/target/wasm32-unknown-unknown/release/multi_wasm.wasm";
            

//     let module = Module::from_file(&engine, path_to_module).unwrap();

//     // For each input spawn a blocking task that instantiates and runs wasm fib
//     let mut store = Store::new(&engine, ());
//     let instance = Instance::new(&mut store, &module, &[]).unwrap();
//     let fib: TypedFunc< u64, u32> = instance.get_typed_func(&mut store, "is_prime").unwrap();

//     let handle = task::spawn_blocking(move || {
//         core_affinity::set_for_current(core6);
//         let result = fib.call(&mut store, 4294967291).unwrap();

//         println!("result, {:?}", result);
//         result
//     });


//     let _result = match handle.await {
//         Ok(result) => Some(result),
//         Err(e) => None,
//     };

// }


// use core_affinity::{get_core_ids, set_for_current, CoreId};
// use std::fmt::Write;
// use wasmtime::{Engine, Module, Store, Instance, TypedFunc};
// use tokio::{self, task};



// #[tokio::main]
// async fn main() {
//     // Set up Wasmtime engine and module outside blocking
//     let engine = Engine::default();
//     let core_ids = get_core_ids().expect("Failed to get core IDs");
//     let core6: CoreId = core_ids[7];
//     let path_to_module: &'static str = "/Users/athanasiapharmake/workspace/fib-host/wasm-modules/target/wasm32-unknown-unknown/release/multi_wasm.wasm";

//     let module = Module::from_file(&engine, path_to_module).unwrap();

//     let handle = task::spawn_blocking(move || {
//         set_for_current(core6);

//         let mut store = Store::new(&engine, ());
//         let instance = Instance::new(&mut store, &module, &[]).unwrap();

//         // Get the exported double_sha256 function
//         let double_sha256: TypedFunc<(i32, i32, i32), ()> =
//             instance.get_typed_func(&mut store, "double_sha256").unwrap();

//         // Get exported memory
//         let memory = instance.get_export(&mut store, "memory")
//             .expect("Memory not found")
//             .into_memory()
//             .expect("Export is not memory");

//         // Input string
//         let input_data = b"bhhbh"; // 100 MB of 'A'
//         let input_ptr = 0;
//         let input_len = input_data.len() as i32;
//         let output_ptr = input_ptr + input_len + 64;


//         // Write input into wasm memory
//         memory.write(&mut store, input_ptr as usize, input_data).unwrap();

//         // Call double_sha256
//         double_sha256.call(&mut store, (input_ptr, input_len, output_ptr)).unwrap();

//         // Read 32-byte result from output_ptr
//         let mut result = [0u8; 32];
//         memory.read(&mut store, output_ptr as usize, &mut result).unwrap();

//         // Format result as hex string
//         let mut hex_output = String::with_capacity(64);
//         for byte in &result {
//             write!(hex_output, "{:02x}", byte).unwrap();
//         }

//         println!("Double SHA256 of {:?}: {}",input_data, hex_output);
//     });

//     let _ = handle.await;
// }


use core_affinity::{get_core_ids, set_for_current, CoreId};
use std::fmt::Write;
use wasmtime::{Engine, Module, Store, Instance, TypedFunc};

fn main() {
    // Set up Wasmtime engine and module
    let engine = Engine::default();
    let core_ids = get_core_ids().expect("Failed to get core IDs");
    let core7: CoreId = core_ids[7];
    println!("core {:?}", core7);
    let path_to_module: &'static str = "/Users/athanasiapharmake/workspace/fib-host/wasm-modules/target/wasm32-unknown-unknown/release/multi_wasm.wasm";

    let module = Module::from_file(&engine, path_to_module).unwrap();

    let handle = std::thread::spawn(move || {
        // Pin this thread to core 7
        let res = set_for_current(core7);
        println!("res: {:?}", res);
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &[]).unwrap();

        // Get the exported double_sha256 function
        let double_sha256: TypedFunc<(i32, i32, i32), ()> =
            instance.get_typed_func(&mut store, "double_sha256").unwrap();

        // Get exported memory
        let memory = instance.get_export(&mut store, "memory")
            .expect("Memory not found")
            .into_memory()
            .expect("Export is not memory");

        // Input string
        let input_data = b"bhhbSFSSSSAdsfsdvsfvfssdvdfvdfsvdsvsdvsddsvdsvdsvsfdvdsh";
        let input_ptr = 0;
        let input_len = input_data.len() as i32;
        let output_ptr = input_ptr + input_len + 64;

        // Write input into wasm memory
        memory.write(&mut store, input_ptr as usize, input_data).unwrap();

        // Call double_sha256
        double_sha256.call(&mut store, (input_ptr, input_len, output_ptr)).unwrap();

        // Read 32-byte result from output_ptr
        let mut result = [0u8; 32];
        memory.read(&mut store, output_ptr as usize, &mut result).unwrap();

        // Format result as hex string
        let mut hex_output = String::with_capacity(64);
        for byte in &result {
            write!(hex_output, "{:02x}", byte).unwrap();
        }

        println!("Double SHA256 of {:?}: {}", input_data, hex_output);
    });

    handle.join().unwrap();
}

