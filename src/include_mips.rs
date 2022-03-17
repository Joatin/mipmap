#[macro_export]
macro_rules! include_mips{
    ($file_name:expr)=>{
        {
            let dir = mipmap::include_dir!("test");
            let mut mip_maps = vec![];
            let mut level = 0;
            while let Ok(file) = dir.get_file(&format!("mip_{}.png", level)) {
                mip_maps.push(file.contents());
                level += 1;
            }

            mip_maps
        }
    }
}