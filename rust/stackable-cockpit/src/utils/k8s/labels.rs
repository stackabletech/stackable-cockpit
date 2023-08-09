use kube::api::ListParams;

pub enum ProductLabel {
    Both,
    Name,
    App,
}

pub trait ListParamsExt {
    fn from_product(
        product_name: &str,
        instance_name: Option<&str>,
        product_label: ProductLabel,
    ) -> ListParams {
        let mut params = ListParams::default();

        if matches!(product_label, ProductLabel::Name | ProductLabel::Both) {
            params.add_label(format!("app.kubernetes.io/name={product_name}"));
        }

        if matches!(product_label, ProductLabel::App | ProductLabel::Both) {
            params.add_label(format!("app.kubernetes.io/app={product_name}"));
        }

        if let Some(instance_name) = instance_name {
            // NOTE (Techassi): This bothers me a little, but .labels consumes self
            params.add_label(format!("app.kubernetes.io/instance={instance_name}"));
        }

        params
    }

    /// Adds a label to the label selectors.
    fn add_label(&mut self, label: impl Into<String>);
}

impl ListParamsExt for ListParams {
    fn add_label(&mut self, label: impl Into<String>) {
        match self.label_selector.as_mut() {
            Some(labels) => labels.push_str(format!(",{}", label.into()).as_str()),
            None => self.label_selector = Some(label.into()),
        }
    }
}
