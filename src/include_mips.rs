#[macro_export]
macro_rules! include_mips{
    ($file_name:expr, for $i:ident in $range:expr)=>{
        {
            let mut mip_maps = vec![];
            for $i in $range {
                mip_maps.push(include_bytes!(concat!(env!("OUT_DIR"), "/", $file_name, "/mip_", $i, ".png")))
            }

            mip_maps
        }
    }
}