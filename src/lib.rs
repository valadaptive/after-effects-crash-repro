use after_effects as ae;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Params { Opacity }

#[derive(Default)]
struct Plugin { }

ae::define_effect!(Plugin, (), Params);

impl AdobePluginGlobal for Plugin {
    fn can_load(_host_name: &str, _host_version: &str) -> bool { true }

    fn params_setup(&self, params: &mut ae::Parameters<Params>, _: ae::InData, _: ae::OutData) -> Result<(), Error> {
        params.add(Params::Opacity, "Opacity", ae::FloatSliderDef::setup(|f| {
            f.set_slider_min(0.0);
            f.set_slider_max(100.0);
            f.set_valid_min(0.0);
            f.set_valid_max(100.0);
        }))
    }

    fn handle_command(&mut self, cmd: ae::Command, in_data: ae::InData, _: ae::OutData, params: &mut ae::Parameters<Params>) -> Result<(), ae::Error> {
        match cmd {
            ae::Command::Render { in_layer, mut out_layer } => {
                let slider_value = params.get(Params::Opacity)?.as_float_slider()?.value();

                let extent_hint = in_data.extent_hint();

                in_layer.iterate_with(&mut out_layer, 0, extent_hint.height(), Some(extent_hint), |_x: i32, _y: i32, pixel: ae::GenericPixel, out_pixel: ae::GenericPixelMut| -> Result<(), Error> {
                    match (pixel, out_pixel) {
                        (ae::GenericPixel::Pixel8(pixel), ae::GenericPixelMut::Pixel8(out_pixel)) => {
                            *out_pixel = *pixel;
                            out_pixel.alpha = (pixel.alpha as f64 * slider_value / 100.0) as u8;
                        }
                        (ae::GenericPixel::Pixel16(pixel), ae::GenericPixelMut::Pixel16(out_pixel)) => {
                            *out_pixel = *pixel;
                            out_pixel.alpha = (pixel.alpha as f64 * slider_value / 100.0) as u16;
                        }
                        _ => return Err(Error::BadCallbackParameter)
                    }
                    Ok(())
                })?;
            }
            ae::Command::UpdateParamsUi => {
                let mut param = params.get(Params::Opacity)?;
                // This plugin will crash if the next line is commented out.
                param = param.clone();
                param.set_ui_flag(ParamUIFlags::DISABLED, true);
                param.update_param_ui()?;
            }
            _ => { }
        }
        Ok(())
    }
}
