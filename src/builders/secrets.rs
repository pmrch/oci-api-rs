use crate::secrets::Secrets;

#[derive(Debug, Default)]
pub struct SecretsBuilder<'a> {
    pub image_id:  Option<&'a str>,
    pub subnet_id: Option<&'a str>,
    pub ssh_key:   Option<&'a str>,
}

impl<'a> SecretsBuilder<'a> {
    #[must_use]
    pub const fn image_id(mut self, image_id: &'a str) -> Self {
        self.image_id = Some(image_id);
        self
    }

    #[must_use]
    pub const fn subnet_id(mut self, subnet_id: &'a str) -> Self {
        self.subnet_id = Some(subnet_id);
        self
    }

    #[must_use]
    pub const fn ssh_key(mut self, ssh_key: &'a str) -> Self {
        self.ssh_key = Some(ssh_key);
        self
    }

    /// Uses provided fields to construct `Secrets`
    #[must_use]
    pub fn build(self) -> Secrets { Secrets::from(self) }
}
