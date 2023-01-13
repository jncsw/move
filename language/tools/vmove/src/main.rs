use move_bytecode_source_map::{source_map::SourceMap};
use std::{fs, path::Path};
use move_binary_format::file_format::CompiledModule;
use move_bytecode_viewer::start_viewer_in_memory;
use std::{fs::File, io::Read};


use anyhow::{format_err, Result};


pub fn source_map_from_file(file_path: &Path) -> Result<SourceMap> {
    let mut bytes = Vec::new();
    File::open(file_path)
        .ok()
        .and_then(|mut file| file.read_to_end(&mut bytes).ok())
        .ok_or_else(|| format_err!("Error while reading in source map information"))?;
    bcs::from_bytes::<SourceMap>(&bytes)
        .map_err(|x| format_err!("Error deserializing into source map: {:?}", x))
}

fn main() {

    

    // let mut base_dir:String = "/home/ubuntu/wormhole/aptos/wormhole/build/Wormhole/".to_string();
    // let mut base_dir:String = "/home/ubuntu/move/language/documentation/tutorial/step_1/BasicCoin/build/BasicCoin/".to_string();

    // let mut module_binary_path = base_dir.to_owned();
    // module_binary_path.push_str("bytecode_modules/BasicCoin.mv");

    // let mut module_sourcemap_path = base_dir.to_owned();
    // module_sourcemap_path.push_str("source_maps/BasicCoin.mvsm");
    // let mut source_file_path = base_dir.to_owned();
    // source_file_path.push_str("sources/BasicCoin.move");

    // let bytecode_bytes =
    //     fs::read(module_binary_path).expect("Unable to read bytecode file");
    // let compiled_module = CompiledModule::deserialize(&bytecode_bytes)
    //     .expect("Module blob can't be deserialized");

    // println!("source_map: {:?}", source_map_from_file(Path::new(&module_sourcemap_path)));

    // let source_map = source_map_from_file(Path::new(&module_sourcemap_path)).unwrap();

    // let source_path = Path::new(&source_file_path);
    // start_viewer_in_memory(compiled_module, source_map, source_path)
}
