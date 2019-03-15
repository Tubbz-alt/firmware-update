use uefi::status::Result;

pub trait Component {
    fn name(&self) -> &str;
    fn path(&self) -> &str;
    fn model(&self) -> &str;
    fn version(&self) -> &str;
    fn validate(&self) -> Result<bool>;
    fn flash(&self) -> Result<()>;
}
