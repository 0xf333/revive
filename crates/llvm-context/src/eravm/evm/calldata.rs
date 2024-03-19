//!
//! Translates the calldata instructions.
//!

use crate::eravm::context::address_space::AddressSpace;
use crate::eravm::context::pointer::Pointer;
use crate::eravm::context::Context;
use crate::eravm::Dependency;
use inkwell::types::BasicType;

///
/// Translates the calldata load.
///
pub fn load<'ctx, D>(
    context: &mut Context<'ctx, D>,
    offset: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency + Clone,
{
    let calldata_pointer = context
        .get_global(crate::eravm::GLOBAL_CALLDATA_POINTER)?
        .value
        .as_pointer_value();
    let offset = context.build_gep(
        Pointer::new(context.byte_type(), AddressSpace::Stack, calldata_pointer),
        &[offset],
        context.field_type().as_basic_type_enum(),
        "calldata_pointer_with_offset",
    );
    context
        .build_load(offset, "calldata_value")
        .map(|value| context.build_byte_swap(value))
}

///
/// Translates the calldata size.
///
pub fn size<'ctx, D>(
    context: &mut Context<'ctx, D>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>>
where
    D: Dependency + Clone,
{
    let value = context.get_global_value(crate::eravm::GLOBAL_CALLDATA_SIZE)?;

    Ok(value)
}

///
/// Translates the calldata copy.
///
pub fn copy<'ctx, D>(
    context: &mut Context<'ctx, D>,
    destination_offset: inkwell::values::IntValue<'ctx>,
    source_offset: inkwell::values::IntValue<'ctx>,
    size: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<()>
where
    D: Dependency + Clone,
{
    let heap_pointer = context
        .get_global(crate::eravm::GLOBAL_HEAP_MEMORY_POINTER)?
        .value
        .as_pointer_value();
    let destination = unsafe {
        context.builder().build_gep(
            context.byte_type(),
            heap_pointer,
            &[destination_offset],
            "calldata_pointer_with_offset",
        )
    }?;

    let calldata_pointer = context
        .get_global(crate::eravm::GLOBAL_CALLDATA_POINTER)?
        .value
        .as_pointer_value();
    let source = unsafe {
        context.builder().build_gep(
            context.byte_type(),
            calldata_pointer,
            &[source_offset],
            "calldata_pointer_with_offset",
        )
    }?;

    context.build_memcpy(
        context.intrinsics().memory_copy_from_generic,
        Pointer::new(context.byte_type(), AddressSpace::Stack, destination),
        Pointer::new(context.byte_type(), AddressSpace::Stack, source),
        size,
        "calldata_copy_memcpy_from_child",
    )
}