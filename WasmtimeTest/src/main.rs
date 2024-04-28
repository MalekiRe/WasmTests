use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use wasi_common::sync::WasiCtxBuilder;
use wasi_common::WasiCtx;
use wasmtime::{Config, Engine, SharedMemory, Store, Module, MemoryType, Linker, Caller, WasmBacktraceDetails};

mod api_bindings;
use api_bindings::{WasmPtr, WasmSlice, ComponentInfo};


#[derive(Copy, Clone)]
#[repr(C)]
pub struct TestComponent {
    pub a: u8,
    pub b: i32,
    pub c: f32
}

fn load_module(path: &str, engine: &Engine) -> Result<Module, String> {
    let file = File::open(path);
    if let Err(e) = file {
        println!("Error opening file: {}", e.to_string());
        return Err(e.to_string());
    }
    let mut buf = Vec::new();
    let mut file = file.unwrap();
    let file_read = file.read_to_end(&mut buf);
    if let Err(e) = file_read {
        println!("Error reading file: {}", e.to_string());
        return Err(e.to_string());
    }
    let now = std::time::Instant::now();
    let module = Module::from_binary(&engine,&buf);
    if module.is_err() {
        let err = module.err().unwrap();
        println!("Error creating module: {}", err.to_string());
        return Err(err.to_string());
    }
    let module = module.unwrap();

    let module_instantiation_time = now.elapsed();
    println!("Module load time: {:?}", module_instantiation_time);
    Ok(module)
}

struct BEState {
    be_state: Arc<BEStateInner>,
    wasi_ctx: Option<WasiCtx>
}

struct BEStateInner {
    engine: Engine,
    memory: SharedMemory,
}

fn be_print_external(caller: Caller<BEState>, text_ptr: u32, size: u32) {
    println!("entered print external");
    let text_ptr = WasmSlice::<u8>::new(text_ptr, size);
    let memory = &caller.data().be_state.memory;
    let text = text_ptr.as_shared_ref(memory);
    println!("Wasm Print: {}", text.as_str());
}

fn main() {
    let use_wasi = true;

    let mut engine_config = Config::new();
    engine_config.wasm_threads(true);
    engine_config.wasm_bulk_memory(true);
    engine_config.debug_info(true);
    engine_config.wasm_backtrace_details(WasmBacktraceDetails::Enable);


    let engine = Engine::new(&engine_config).unwrap();
    let main_memory = SharedMemory::new(&engine, MemoryType::shared(50, 32768)).unwrap();

    let wasi_ctx = if use_wasi {
        Some(WasiCtxBuilder::new().inherit_stdio().build())
    } else {
        None
    };

    let data1 = BEState{
        be_state: Arc::new(BEStateInner {
            engine: engine.clone(),
            memory: main_memory.clone()
        }),
        wasi_ctx
    };
    let wasi_ctx = if use_wasi {
        Some(WasiCtxBuilder::new().inherit_stdio().build())
    } else {
        None
    };
    let data2 = BEState{
        be_state: Arc::new(BEStateInner {
            engine: engine.clone(),
            memory: main_memory.clone()
        }),
        wasi_ctx
    };
    let mut store1 = Store::new(&engine, data1);
    let mut store2 = Store::new(&engine, data2);

    // Load two different modules importing the same shared memory
    let module1 = load_module("wasm_crate1.wasm", &engine).unwrap();
    let module2 = load_module("wasm_crate2.wasm", &engine).unwrap();


    let mut linker1 = Linker::new(&engine);
    linker1.define(&mut store1, "env", "memory", main_memory.clone()).expect("Could not define memory for store1");
    linker1.func_wrap("BraneEngine", "extern_be_print", be_print_external).unwrap();

    let mut linker2  = Linker::new(&engine);
    linker2.define(&mut store2, "env", "memory", main_memory.clone()).expect("Could not define memory for store2");
    linker2.func_wrap("BraneEngine", "extern_be_print", be_print_external).unwrap();

    if use_wasi {
        wasi_common::sync::add_to_linker(&mut linker1, |s: &mut BEState| {
            s.wasi_ctx.as_mut().expect("no wasi context")
        }).expect("Could not add WASI to linker1");
        wasi_common::sync::add_to_linker(&mut linker2, |s: &mut BEState| {
            s.wasi_ctx.as_mut().expect("no wasi context")
        }).expect("Could not add WASI to linker2");
    }

    {
        println!("-----Module 1-----");
        let now = std::time::Instant::now();
        let instance1 = linker1.module(&mut store1, "module1", &module1);
        if instance1.is_err() {
            println!("Was unable to instantiate module1: {:?}", instance1.err().unwrap());
            return;
        }
        let instanceInstantiationTime = now.elapsed();
        println!("Instance1 instantiation time: {:?}", instanceInstantiationTime);

        println!("Imports");
        let imports = module1.imports();
        for i in imports {
            println!("{:?}: {}", i.ty(), i.name());
        }

        println!("Exports");
        let exports = module1.exports();
        for e in exports {
            println!("{:?}: {}", e.ty(), e.name());
        }

        println!("-----Module 1, store 2-----");
        let now = std::time::Instant::now();
        let instance2 = linker2.module(&mut store2, "module1", &module1);
        if instance2.is_err() {
            println!("Was unable to instantiate module1: {:?}", instance2.err().unwrap());
            return;
        }
        let instanceInstantiationTime = now.elapsed();
        println!("Instance2 instantiation time: {:?}", instanceInstantiationTime);
    }




    println!("-----Module 2-----");
    {
        let now = std::time::Instant::now();
        let instance1 = linker1.module(&mut store1, "module2", &module2);
        if instance1.is_err() {
            println!("Was unable to instantiate module2: {:?}", instance1.err().unwrap());
            return;
        }
        instance1.expect("Couldn't instantiate module2");
        let instanceInstantiationTime = now.elapsed();
        println!("Instance2 instantiation time: {:?}", instanceInstantiationTime);

        let instance2 = linker2.module(&mut store2, "module2", &module2);
        if instance2.is_err() {
            println!("Was unable to instantiate module2: {:?}", instance2.err().unwrap());
            return;
        }
        instance2.expect("Couldn't instantiate module2");

        println!("Imports");
        let imports = module2.imports();
        for i in imports {
            println!("{:?}: {}", i.ty(), i.name());
        }

        println!("Exports");
        let exports = module2.exports();
        for e in exports {
            println!("{:?}: {}", e.ty(), e.name());
        }
    }
    println!("-----end-----");

    let create_world = linker1.get(&mut store1, "module1", "create_world").unwrap().into_func().unwrap().typed::<(), u32, >(&store1).unwrap();
    let create_app = linker1.get(&mut store1, "module1", "create_app").unwrap().into_func().unwrap().typed::<(), u32, >(&store1).unwrap();
    let tick_app = linker1.get(&mut store1, "module1", "tick_app").unwrap().into_func().unwrap().typed::<u32, u32>(&store1).unwrap();



    let app1 = create_app.call(&mut store1, ()).unwrap();

    let mut world_ptr;

    match create_world.call(&mut store1, ()) {
        Ok(res2) => {
            world_ptr = res2;
        }
        Err(err) => {
            eprintln!("create world failed: {}", err.to_string());
            return;
        }
    }

    let test_world_access = linker2.get(&mut store2, "module2", "test_world_access").expect("could not find test_world_access").into_func().unwrap().typed::<u32, u32>(&store2).unwrap();
    let module_2_tick = linker2.get(&mut store2, "module2", "tick").unwrap().into_func().unwrap().typed::<u32, ()>(&store2).unwrap();

    let mut app2 = test_world_access.call(&mut store2, world_ptr).unwrap();

    for _ in 0..5 {
        tick_app.call(&mut store1, app1).unwrap();
        module_2_tick.call(&mut store2, app2).unwrap();
    }

}
