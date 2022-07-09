use strong_xml::{XmlRead, XmlWrite};

use super::latent_style::LatentStyle;

/// Styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
///
/// ```rust
/// use docx::styles::*;
///
/// let style = Styles::new()
///     .default(DefaultStyle::default())
///     .push(Style::new(StyleType::Paragraph, "style_id"));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:latentStyles")]
pub struct LatentStyles<'a> {
    /// Specifies a set of properties.
    #[xml(attr = "w:defLockedState")]
    pub locked_state: Option<usize>,
    #[xml(attr = "w:defUIPriority")]
    pub priority: Option<usize>,
    #[xml(attr = "w:defSemiHidden")]
    pub semi_hidden: Option<usize>,
    #[xml(attr = "w:defUnhideWhenUsed")]
    pub unhide_when_used: Option<usize>,
    #[xml(attr = "w:defQFormat")]
    pub format: Option<usize>,
    #[xml(attr = "w:count")]
    pub count: Option<usize>,
    #[xml(child = "w:lsdException")]
    pub styles: Vec<LatentStyle<'a>>,
}

impl<'a> LatentStyles<'a> {
    pub fn push(&mut self, style: LatentStyle<'a>) -> &mut Self {
        self.styles.push(style);
        self.count = Some(self.styles.len());
        self
    }
}
