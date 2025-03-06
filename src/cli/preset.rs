use clap::ValueEnum;
use minimap2::Preset;

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum PresetWrapper {
    LrHqae,
    LrHq,
    Splice,
    SpliceHq,
    Asm,
    Asm5,
    Asm10,
    Asm20,
    Sr,
    MapPb,
    MapHifi,
    MapOnt,
    AvaPb,
    AvaOnt,
    Short,
    Map10k,
    Cdna,
}
impl From<PresetWrapper> for Preset {
    fn from(value: PresetWrapper) -> Self {
        match value {
            PresetWrapper::LrHqae => Preset::LrHqae,
            PresetWrapper::LrHq => Preset::LrHq,
            PresetWrapper::Splice => Preset::Splice,
            PresetWrapper::SpliceHq => Preset::SpliceHq,
            PresetWrapper::Asm => Preset::Asm,
            PresetWrapper::Asm5 => Preset::Asm5,
            PresetWrapper::Asm10 => Preset::Asm10,
            PresetWrapper::Asm20 => Preset::Asm20,
            PresetWrapper::Sr => Preset::Sr,
            PresetWrapper::MapPb => Preset::MapPb,
            PresetWrapper::MapHifi => Preset::MapHifi,
            PresetWrapper::MapOnt => Preset::MapOnt,
            PresetWrapper::AvaPb => Preset::AvaPb,
            PresetWrapper::AvaOnt => Preset::AvaOnt,
            PresetWrapper::Short => Preset::Short,
            PresetWrapper::Map10k => Preset::Map10k,
            PresetWrapper::Cdna => Preset::Cdna,
        }
    }
}
