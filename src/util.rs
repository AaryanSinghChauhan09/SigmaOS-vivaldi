//! vivaldi model — web browser.

#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    pub url: String,
    pub active: bool,
    pub pinned: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub tabs: Vec<Tab>,
}

impl Default for Model {
    fn default() -> Self { Self { tabs: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, title: &str, url: &str) {
        self.tabs.push(Tab { title: title.into(), url: url.into(), active: true, pinned: false });
    }

    pub fn pin(&mut self, index: usize) -> bool {
        if let Some(t) = self.tabs.get_mut(index) { t.pinned = true; true } else { false }
    }

    pub fn list_tabs(&self) -> &[Tab] { &self.tabs }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_pin() {
        let mut v = Model::new();
        v.open("Example", "https://example.com");
        assert!(v.pin(0));
        assert!(v.list_tabs()[0].pinned);
    }
}
