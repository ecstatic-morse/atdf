use svd_parser as svd;
use super::schema;

/// Unsafe initializer for the various structs from the `svd_parser` crate.
///
/// These are sealed with a zero-size private field, but make all other fields public.
macro_rules! svd_init {
    (
        $T:ty {
            $( $field:ident : $value:expr ),* $(,)*
        }
    ) => {
        unsafe {
            let mut ret: $T =  ::std::mem::uninitialized();
            $(
                ::std::ptr::write(&mut ret.$field, $value);
             )*
            ret
        }
    };
}

pub fn device(dev: schema::Device, modules: &[schema::Module]) -> svd::Device {
    let defaults = svd_init! {
        svd::Defaults {
            size: None,
            reset_value: None,
            reset_mask: None,
            access: None,
        }
    };

    let mut peripherals = vec![];
    for family in dev.peripherals() {
        for p in &family.instances {
            if p.registers.as_ref().map_or(false, |r| r.address_space != "data") {
                continue
            }

            let p = peripheral(p.clone(), &family.name, modules);
            peripherals.push(p);
        }
    }

    svd_init!{
        svd::Device {
            name: dev.name,
            cpu: None,
            peripherals: peripherals,
            defaults: defaults,
        }
    }
}

fn peripheral(p: schema::Peripheral, group: &str, modules: &[schema::Module]) -> svd::Peripheral {
    let registers = p.registers.unwrap();
    let base_address = registers.offset;

    let module = modules.iter().find(|m| m.name == group).unwrap();
    let group = module.register_groups.iter().find(|r| r.name == registers.name_in_module).unwrap();
    let registers = group.registers
        .iter()
        .cloned()
        .map(|r| register(r, module))
        .collect();

    svd_init!{
        svd::Peripheral {
            name: p.name,
            group_name: None,
            description: Some(p.description),
            base_address: base_address,
            interrupt: vec![],
            registers: Some(registers),
            derived_from: None,
        }
    }
}

impl From<schema::Access> for svd::Access {
    fn from(acc: schema::Access) -> Self {
        match acc {
            schema::Access::ReadOnly => svd::Access::ReadOnly,
            schema::Access::WriteOnly => svd::Access::WriteOnly,
            schema::Access::ReadWrite
                | schema::Access::Bidirectional
                => svd::Access::ReadWrite,
        }
    }
}

fn register(r: schema::Register, module: &schema::Module) -> svd::Register {
    let access = r.access.map(Into::into);

    let fields = r.bitfields
        .iter()
        // TODO: handle non-contiguous fields
        .filter(|f| bit_range(f.mask).is_some())
        .map(|f| field(f.clone(), module))
        .collect();

    let reg = svd_init!{
        svd::RegisterInfo {
            name: r.name,
            description: r.description,
            address_offset: r.offset,
            size: Some(r.size * 8), // TODO: check units (bytes?)
            access: access,
            reset_value: None,
            reset_mask: None,
            fields: Some(fields),
            write_constraint: None,
        }
    };

    svd::Register::Single(reg)
}

fn field(f: schema::Bitfield, module: &schema::Module) -> svd::Field {
    let range = bit_range(f.mask).unwrap();

    let values = f.values
        .map(|v| module.value_groups.iter().find(|vg| vg.name == v).unwrap())
        .map(|vg| value_group(vg.clone()));

   svd_init!{
        svd::Field {
            name: f.name,
            description: Some(f.description),
            bit_range: range,
            access: None,
            enumerated_values: values.map_or_else(Default::default, |v| vec![v]),
            write_constraint: None,
        }
    }
}

fn value_group(vg: schema::ValueGroup) -> svd::EnumeratedValues {
    let values = vg.values
        .iter()
        .map(|v| {
            svd_init! {
                svd::EnumeratedValue {
                    name: v.name.clone(),
                    description: Some(v.description.clone()),
                    value: Some(v.value),
                    is_default: None,
                }
            }
        })
        .collect();

    svd_init! {
        svd::EnumeratedValues {
            name: Some(vg.name.clone()),
            usage: None,
            derived_from: None,
            values: values,
        }
    }
}

fn bit_range(mask: u32) -> Option<svd::BitRange> {
    assert!(mask != 0);

    let start = mask.trailing_zeros();
    let mut shifted = mask as u64 >> start;
    shifted += 1;
    if shifted.count_ones() != 1 {
        return None;
    }

    let len = shifted.trailing_zeros();
    Some(svd::BitRange {
        offset: start,
        width: len,
    })
}
