use {crate::{Channel, RGBA}, gtk::traits::ColorChooserExt};

impl<T> From<T> for RGBA<f32>
where T: ColorChooserExt {
    fn from(widget: &impl ColorChooserExt) -> Self {
        widget.rgba().into()
    }
}
