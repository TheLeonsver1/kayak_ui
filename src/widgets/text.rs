use crate::core::{
    render_command::RenderCommand,
    styles::{Style, StyleProp, Units},
    widget,
};

#[widget]
pub fn Text(size: f32, content: String, styles: Option<Style>, font: Option<u16>) {
    let px_size = {
        let mut px_size = (0.0, 0.0);
        #[cfg(feature = "bevy_renderer")]
        {
            let font = context.query_world::<(
                bevy::prelude::Res<bevy_kayak_ui::FontMapping>,
                bevy::prelude::Res<bevy::prelude::Assets<kayak_font::KayakFont>>,
            ), _, _>(|(font_mapping, kayak_fonts)| {
                kayak_fonts
                    .get(font_mapping.get_handle(font.unwrap_or(0)).unwrap())
                    .and_then(|value| Some(value.clone()))
            });

            if let Some(font) = font {
                px_size = context.measure_string(&font, &content, size);
            }
        }
        px_size
    };
    let render_command = RenderCommand::Text {
        content,
        size,
        font: font.unwrap_or(0),
    };
    *styles = Some(Style {
        render_command: StyleProp::Value(render_command),
        width: if px_size.0 == 0.0 {
            StyleProp::default()
        } else {
            StyleProp::Value(Units::Pixels(px_size.0))
        },
        height: if px_size.1 == 0.0 {
            StyleProp::default()
        } else {
            StyleProp::Value(Units::Pixels(px_size.1))
        },
        ..styles.clone().unwrap_or_default()
    });
}