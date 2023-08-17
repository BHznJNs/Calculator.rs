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
