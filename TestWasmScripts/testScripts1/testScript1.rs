#![feature(raw_ref_op)]

use std::time::Duration;
use bevy_app::{App, AppExit, FixedMain, FixedMainScheduleOrder, Main, MainScheduleOrder, MainSchedulePlugin, RunFixedMainLoop, ScheduleRunnerPlugin, Startup, Update};
use bevy_ecs::intern::Interned;
use bevy_ecs::prelude::{AppTypeRegistry, Component, Query, Schedule, Schedules};
use bevy_ecs::schedule::{ExecutorKind, ScheduleLabel};
use bevy_ecs::world::{FromWorld, World};
use bevy_ecs::system::Commands;
use bevy_transform::prelude::Transform;

#[link(wasm_import_module = "BraneEngine")]
extern "C" {
    pub fn extern_be_print(msg: *const u8, size: u32);
}

pub fn be_print(msg: &str) {
    unsafe {
        extern_be_print(msg.as_ptr(), msg.len() as u32);
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct TestComponent {
    pub a: bool,
    pub b: i32,
    pub c: f32
}

#[no_mangle]
pub extern "C" fn test_function(ret: i32) -> i32 {
    be_print(format!("Test function passed {}", ret).as_str());
    ret
}

#[no_mangle]
pub extern "C" fn create_app() -> *mut App {
    //world.insert_resource(Schedules::default());
    let mut app = App::new();
    app.add_plugins(bevy_app::ScheduleRunnerPlugin::run_loop(Duration::from_secs(1)));
    app.add_systems(Startup, spawn_thing);
    app.add_systems(Update, change_transform);
    app.add_systems(Update, system_print2);
    Box::leak(Box::new(app))
}

static mut APP: Option<&'static mut App> = None;

#[no_mangle]
pub extern "C" fn create_world() -> *mut World {
    unsafe { std::mem::transmute(APP.as_mut().unwrap().world_mut()) }
}

#[no_mangle]
pub extern "C" fn get_update() -> *mut Interned<dyn ScheduleLabel> {
    Box::leak(Box::new(Update.intern()))
}

fn change_transform(mut query: Query<&mut Transform>) {
    for mut transform in query.iter_mut() {
        transform.translation.x += 1.0;
    }
}

fn spawn_thing(mut commands: Commands) {
    let e = commands.spawn(Transform::from_xyz(69420.0, 1.0, 1.0)).id();
    println!("entity is: {}", e.index());
}

#[no_mangle]
pub extern "C" fn tick_app(app: *mut App) -> *mut App {
    let app = unsafe {&mut *app};

    app.update();

    app
}

fn system_print2() {
    println!("owo");
}

#[no_mangle]
pub extern "C"  fn create_test_component() -> *mut TestComponent {
    let component = Box::new(TestComponent {
        a: false,
        b: 0,
        c: 0f32
    });
    Box::into_raw(component)
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

