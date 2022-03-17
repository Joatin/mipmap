#[macro_export]
macro_rules! include_mips{
    ($file_name:expr, $range:expr)=>{
        {
            let mut mip_maps = vec![];
            for $level in $range {
                mip_maps.push(include_bytes!(concat!(env!("OUT_DIR"), "/", $file_name, "/mip_", $level, ".png")))
            }

            mip_maps
        }
    }
}