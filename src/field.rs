use crate::import::{load_u16};
use crate::attribute::Attribute;
use crate::import::AccessFlags;
use crate::import::ConstantItem;
#[derive(Debug)]
pub struct Field {
    pub(crate) flags: AccessFlags,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    attributes: Box<[Attribute]>,
}
impl Field {
    pub(crate) fn read<R: std::io::Read>(
        src: &mut R,
        const_items: &[ConstantItem],
    ) -> Result<Self, std::io::Error> {
        let flags = AccessFlags::read(src)?;
        let name_index = load_u16(src)?;
        let descriptor_index = load_u16(src)?;
        let attributes_count = load_u16(src)?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Attribute::read(src, const_items)?);
        }
        Ok(Self {
            flags,
            name_index,
            descriptor_index,
            attributes: attributes.into(),
        })
    }
}
