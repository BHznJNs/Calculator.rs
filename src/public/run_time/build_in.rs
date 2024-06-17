use std::fmt;

use crate::public::std::modules::{
    array::ArrayModule, basic::BasicModule, bit_ops::BitOpsModule, file_system::FileSysModule,
    map::MapModule, math::MathModule, string::StringModule,
};

#[derive(PartialEq, Clone)]
pub enum BuildInFnIdenti {
    Basic(BasicModule),
    Math(MathModule),
    Array(ArrayModule),
    String(StringModule),
    Map(MapModule),
    FileSystem(FileSysModule),
    BitOps(BitOpsModule),
}

impl fmt::Display for BuildInFnIdenti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildInFnIdenti::Basic(basic_mod) => write!(f, "`{:?}` from Basic", basic_mod),
            BuildInFnIdenti::Math(math_mod) => write!(f, "{:?} from Math", math_mod),
            BuildInFnIdenti::Array(array_mod) => write!(f, "{:?} from Array", array_mod),
            BuildInFnIdenti::String(string_mod) => write!(f, "{:?} from String", string_mod),
            BuildInFnIdenti::Map(map_mod) => write!(f, "{:?} from Map", map_mod),
            BuildInFnIdenti::FileSystem(fs_mod) => write!(f, "{:?} from FS", fs_mod),
            BuildInFnIdenti::BitOps(bitops_mod) => write!(f, "{:?} from BitOps", bitops_mod),
        }
    }
}
