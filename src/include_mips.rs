#[macro_export]
macro_rules! include_mips{
    ($file_name:expr, $levels:expr)=>{
        {
            let mut mip_maps = vec![];
            for level in 0..$levels {
                mip_maps.push(include_bytes!(concat!(env!("OUT_DIR"), format!("/{}/mip_{}.png", $file_name, level))))
            }

            mip_maps
        }
    }
}