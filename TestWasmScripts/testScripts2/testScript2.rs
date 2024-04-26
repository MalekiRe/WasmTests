#![feature(raw_ref_op)]


use std::collections::HashMap;
use std::ptr::addr_of;
use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Entity, Query};
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

/*#[link(wasm_import_module = "BraneEngine")]
extern "C" {
    pub fn extern_be_print(msg: *const u8, size: u32);
}

pub fn be_print(msg: &str) {
    unsafe {
        extern_be_print(msg.as_ptr(), msg.len() as u32);
    }
}
*/
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
pub extern "C" fn test_world_access(mut world: *mut World) -> i32 {

    let mut val = 0;
    /*let e = match unsafe { (*world).get::<Transform>(Entity::from_raw(0)) } {
        None => return 1,
        Some(e) => e,
    };
    val = e.translation.x as i32;*/

    let mut ptr = (&mut val as *mut i32);
    let mut ptr = unsafe { (&mut *ptr) };

    println!("owo");

    if (world.is_null()) {
        println!("was null");
        return 1;
    }
    println!("uwu");

    unsafe { (*world).run_system_once(|query: Query<&Transform>| {
        for transform in query.iter() {
            *ptr = transform.translation.x as i32;
        }
    }) }


    //REMOVE the line above to see the non-buggy behavior.

    val
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

