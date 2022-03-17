#[macro_export]
macro_rules! include_mips{
    ($file_name:expr)=>{
        {
            let dir = include_dir::include_dir!(format!("{}/{}/", env!("OUT_DIR"), $file_name))
            let mut mip_maps = vec![];
            let mut level = 0;
            while Ok(file) = dir.get_file(&format!("mip_{}.png", level)) {
                mip_maps.push(file.contents());
                level += 1;
            }

            mip_maps
        }
    }
}