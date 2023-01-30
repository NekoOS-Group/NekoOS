pub fn get_fdt(data: &[u8]) -> fdt::Fdt {
    let fdt = fdt::Fdt::new(data).unwrap();
    debug!("This is a devicetree representation of a {}", fdt.root().model());
    debug!("...which is compatible with at least: {}", fdt.root().compatible().first());
    debug!("...and has {} CPU(s)", fdt.cpus().count());
    debug!(
        "...and has at least one memory location at: {:#X}\n",
        fdt.memory().regions().next().unwrap().starting_address as usize
    );

    let chosen = fdt.chosen();
    if let Some(bootargs) = chosen.bootargs() {
        debug!("The bootargs are: {:?}", bootargs);
    }

    if let Some(stdout) = chosen.stdout() {
        debug!("It would write stdout to: {}", stdout.name);
    }

    let soc = fdt.find_node("/soc");
    debug!("Does it have a `/soc` node? {}", if soc.is_some() { "yes" } else { "no" });
    if let Some(soc) = soc {
        debug!("...and it has the following children:");
        for child in soc.children() {
            debug!("    {}", child.name);
        }
    }
    fdt
}