use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref RENDERER: Tera = {
        let mut renderer = Tera::default();

        renderer.add_raw_templates(vec![(include_str!("templates/"))]);

        renderer
    };
}

pub struct OutputRenderer {}
