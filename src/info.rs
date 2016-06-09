use std::collections::HashMap;
use std::path::PathBuf;

use glorious::Color;

use spec::*;

fn parse_color(text: &str) -> Result<Color, String> {
    if text.len() != 6 {
        return Err(format!("color string must have length 6 (was {})", text.len()));
    }
    let rgb = u32::from_str_radix(text, 16).map_err(|e| format!("{}", e))?;
    Ok(Color((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8, 0xff))
}

#[derive(Clone, Debug)]
pub struct SpriteInfo {
    pub texture: String,
    pub area: Option<(u32, u32, u32, u32)>,
}

impl SpriteInfo {
    #[inline]
    fn from_spec(spec: SpriteSpec) -> Result<SpriteInfo, String> {
        Ok(SpriteInfo {
            texture: spec.texture,
            area: spec.area,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Schema {
    pub name: String,
    pub layers: Vec<String>,
    pub prefix: PathBuf,
    pub colors: Vec<Color>,
    pub tiles: HashMap<String, SpriteInfo>,
}

impl Schema {
    #[inline]
    pub fn from_spec(spec: SchemaSpec) -> Result<Schema, String> {
        Ok(Schema {
            name: spec.name,
            layers: spec.layers,
            prefix: spec.prefix,
            colors: spec.colors.into_iter().map(|s| parse_color(&s)).collect()?,
            tiles: spec.tiles
                .into_iter()
                .map(|(k, v)| Ok((k, SpriteInfo::from_spec(v)?)))
                .collect::<Result<_, String>>()?,
        })
    }
}
