#![feature(raw_ref_op)]


use std::time::Duration;
use bevy_app::{App, AppExit, FixedMain, FixedMainScheduleOrder, Main, MainScheduleOrder, MainSchedulePlugin, RunFixedMainLoop, SubApp, Update};
use bevy_derive::AppLabel;
use bevy_ecs::prelude::{AppTypeRegistry, FromWorld, Query, Schedule, Schedules};
use bevy_ecs::schedule::{ExecutorKind, ScheduleLabel};
use bevy_ecs::system::RunSystemOnce;
use bevy_ecs::world::World;
use bevy_transform::prelude::Transform;

#[derive(Clone)]
#[repr(C)]
pub struct TestComponent {
    pub a: bool,
    pub b: i32,
    pub c: f32
}

// #[link(wasm_import_module = "BraneEngine")]
// extern "C" {
//     pub fn extern_be_print(msg: *const u8, size: u32);
// }
//
// pub fn be_print(msg: &str) {
//     unsafe {
//         extern_be_print(msg.as_ptr(), msg.len() as u32);
//     }
// }

#[no_mangle]
pub extern "C" fn test_component_access(mut component: Box<TestComponent>) -> Box<TestComponent> {
    *component = TestComponent {
        a: true,
        b: 42,
        c: 3.14
    };
    component
}

#[no_mangle]
pub extern "C" fn test_world_access(mut world: Box<World>) -> *mut App {
    //world.insert_resource(Schedules::default());
    let mut app2 = App::new();
    app2.insert_sub_app(MyApp, SubApp::new());
    *app2.world_mut() = *world;

    Box::leak(Box::new(app2))
}


#[derive(AppLabel, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MyApp;

#[no_mangle]
pub extern "C" fn tick(mut app: *mut App) {
    //app.sub_app(MyApp);
    unsafe { (&mut *app).world_mut() } .run_system_once(print_transform);
}

fn print_transform(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("{:#?}", transform);
    }
}





// ----------------- Expanded macros from brane_engine_api_macros -----------------
#[repr(align(4))]
#[repr(C)]
pub struct ComponentFieldInfo {
    pub offset: u32,
    pub size: u32,
    pub name: &'static str,
    pub ty: &'static str,

}

#[repr(align(4))]
#[repr(C)]
pub struct ComponentInfo {
    pub size: u32,
    pub fields: &'static [ComponentFieldInfo]
}

#[allow(non_upper_case_globals)]
static be_info_data_TestComponent: ComponentInfo =
    ComponentInfo {
        size: std::mem::size_of::<TestComponent>() as u32,
        fields: &[ComponentFieldInfo {
            offset:
            {
                let uninit =
                    memoffset::__priv::mem::MaybeUninit::<TestComponent>::uninit();
                let base_ptr: *const TestComponent = uninit.as_ptr();
                let field_ptr =
                    {
                        #[allow(clippy :: unneeded_field_pattern)]
                            let TestComponent { a: _, .. };
                        let base = base_ptr;

                        #[allow(unused_unsafe)]
                        unsafe {
                            { &raw const (*(base as *const TestComponent)).a }
                        }
                    };
                {
                    let field = field_ptr;
                    let base = base_ptr;
                    unsafe {
                        (field as *const u8).offset_from(base as *const u8) as usize
                    }
                }
            } as u32,
            size: std::mem::size_of::<bool>() as u32,
            name: "a",
            ty: "bool",
        },
            ComponentFieldInfo {
                offset: {
                    let uninit =
                        ::memoffset::__priv::mem::MaybeUninit::<TestComponent>::uninit();
                    let base_ptr: *const TestComponent = uninit.as_ptr();
                    let field_ptr =
                        {
                            #[allow(clippy :: unneeded_field_pattern)]
                                let TestComponent { b: _, .. };
                            let base = base_ptr;

                            #[allow(unused_unsafe)]
                            unsafe {
                                { &raw const (*(base as *const TestComponent)).b }
                            }
                        };
                    {
                        let field = field_ptr;
                        let base = base_ptr;
                        unsafe {
                            (field as *const u8).offset_from(base as *const u8) as usize
                        }
                    }
                } as u32,
                size: std::mem::size_of::<i32>() as u32,
                name: "b",
                ty: "i32",
            },
            ComponentFieldInfo {
                offset: {
                    let uninit = memoffset::__priv::mem::MaybeUninit::<TestComponent>::uninit();
                    let base_ptr: *const TestComponent = uninit.as_ptr();
                    let field_ptr =
                        {
                            #[allow(clippy :: unneeded_field_pattern)]
                                let TestComponent { c: _, .. };
                            let base = base_ptr;

                            #[allow(unused_unsafe)]
                            unsafe {
                                { &raw const (*(base as *const TestComponent)).c }
                            }
                        };
                    {
                        let field = field_ptr;
                        let base = base_ptr;
                        unsafe {
                            (field as *const u8).offset_from(base as *const u8) as usize
                        }
                    }
                } as u32,
                size: std::mem::size_of::<f32>() as u32,
                name: "c",
                ty: "f32",
            }],
    };

#[no_mangle]
pub extern "C" fn be_info_TestComponent()
    -> *const ComponentInfo {
    unsafe {
        &be_info_data_TestComponent as *const ComponentInfo
    }
}
#[no_mangle]
pub extern "C" fn be_clone_TestComponent(dest: *mut TestComponent,
                                         src: *const TestComponent) {
    unsafe { *dest = (*src).clone(); }
}
#[no_mangle]
pub extern "C" fn be_drop_TestComponent(component: *mut TestComponent) {
    unsafe { let data = Box::from_raw(component); drop(data); }
}
// ----------------- End expanded macros from brane_engine_api_macros -----------------

