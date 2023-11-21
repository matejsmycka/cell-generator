use std::io;
use std::mem::size_of;
use toy_arms::external::module::Module;
use toy_arms::external::process::Process;
use toy_arms::external::{read, write};

fn follow_ptr_chain_u32(proc: &Process, base: usize, offsets: &[usize]) -> usize {
    let mut buffer: usize = 0;
    let _ = read(
        &proc.handle,
        base as usize,
        size_of::<u32>(),
        &mut buffer as *mut usize,
    );

    for &offset in &offsets[0..offsets.len() - 1] {
        let _ = read(
            &proc.handle,
            buffer + offset,
            size_of::<u32>(),
            &mut buffer as *mut usize,
        );
    }

    buffer + offsets.last().cloned().unwrap_or(0)
}
fn main() {
    let proc;
    match Process::from_process_name("deadcells.exe") {
        Ok(p) => proc = p,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    let client: Module;
    match proc.get_module_info("libhl.dll") {
        Ok(m) => client = m,
        Err(e) => {
            println!("{}", e);
            return;
        }
    }

    let module_addr = client.base_address + 0x00048184;
    let offsets = [0x6B8, 0x4, 0x18, 0x68, 0x480, 0xC, 0x358].to_vec();

    let cell_counter_ptr: usize = follow_ptr_chain_u32(&proc, module_addr, &offsets);
    let mut cell_counter_value: u32 = 0;
    let _ = read(
        &proc.handle,
        cell_counter_ptr as usize,
        size_of::<u32>(),
        &mut cell_counter_value as *mut u32,
    );

    println!("Current cell counter value: {:?}", cell_counter_value);
    println!("Enter a new value:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut new_value: u32 = input.trim().parse().unwrap();
    let new_value_ptr = &mut new_value;
    let _ = write(&proc.handle, cell_counter_ptr, new_value_ptr);
    println!("Cell counter UI will be updated after sub/add cell ingame event.")
}
