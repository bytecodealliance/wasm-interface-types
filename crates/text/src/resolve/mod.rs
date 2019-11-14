use crate::ast::*;
use wast::Error;

mod expand;
mod names;
mod tyexpand;

pub fn resolve<'a>(
    core: &[wast::ModuleField<'a>],
    adapters: &mut Vec<Adapter<'a>>,
    names: &wast::Names<'a>,
) -> Result<(), Error> {
    // Expanding inline annotations
    let mut expander = expand::Expander::default();
    expander.process(adapters, expand::Expander::deinline_import)?;
    expander.process(adapters, |e, a| e.deinline_non_import(a, core))?;

    // Expanding inline type annotations
    let mut cur = 0;
    let mut expander = tyexpand::Expander::default();
    move_types_first(adapters);
    while cur < adapters.len() {
        expander.expand(&mut adapters[cur]);
        for new in expander.to_prepend.drain(..) {
            adapters.insert(cur, new);
            cur += 1;
        }
        cur += 1;
    }

    // Name resolution of adapters
    move_imports_first(adapters);
    let mut resolver = names::Resolver::new(names);
    for adapter in adapters.iter_mut() {
        resolver.register(adapter);
    }
    for adapter in adapters.iter_mut() {
        resolver.resolve(adapter)?;
    }
    Ok(())
}

fn move_imports_first(adapters: &mut [Adapter<'_>]) {
    adapters.sort_by_key(|f| match f {
        Adapter::Import(_) => false,
        _ => true,
    });
}

fn move_types_first(adapters: &mut [Adapter<'_>]) {
    adapters.sort_by_key(|f| match f {
        Adapter::Type(_) => false,
        _ => true,
    });
}
