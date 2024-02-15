fn some_fn() {}

const ASSERT_SIZE_OF_TYPE_ID_IS_128_BITS: () = {
    if core::mem::size_of::<core::any::TypeId>() != core::mem::size_of::<u128>() {
        panic!("core::any::TypeId is not 128 bits");
    }
};

struct Type<const N: u128>;

const fn type_id_of_val_as_u128<T>(_: &T) -> u128 {
    let type_id = core::any::TypeId::of::<T>();
    unsafe { core::mem::transmute::<core::any::TypeId, u128>(type_id) }
}

type SomeFnType = Type<{ type_id_of_val_as_u128(&some_fn) }>;

fn main() {
    println!("Hello, world!");
}
