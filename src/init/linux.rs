#[no_mangle]
pub extern "C" fn constructor() {

}

#[no_mangle]
pub extern "C" fn destructor() {

}

#[used]
#[link_section = ".init_array"]
static INIT_ARRAY: extern "C" fn() = constructor;

#[used]
#[link_section = ".fini_array"]
static FINI_ARRAY: extern "C" fn() = destructor;