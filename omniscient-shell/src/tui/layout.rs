//! Layout management

use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct LayoutManager {
    // Layout logic
}

impl LayoutManager {
    pub fn new() -> Self {
        LayoutManager {}
    }

    /// Calculate vertical split layout
    /// TODO: Add support for custom constraints and dynamic resizing
    pub fn vertical_split(&self, area: Rect, percentages: &[u16]) -> Vec<Rect> {
        let constraints: Vec<Constraint> = percentages
            .iter()
            .map(|&p| Constraint::Percentage(p))
            .collect();

        Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area)
            .to_vec()
    }

    /// Calculate horizontal split layout
    /// TODO: Add support for minimum/maximum sizes and gaps
    pub fn horizontal_split(&self, area: Rect, percentages: &[u16]) -> Vec<Rect> {
        let constraints: Vec<Constraint> = percentages
            .iter()
            .map(|&p| Constraint::Percentage(p))
            .collect();

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area)
            .to_vec()
    }

    /// Get default 4-pane layout (shell, agent, preview, log)
    /// TODO: Make layout configurable from user preferences
    pub fn default_layout(&self, area: Rect) -> [Rect; 4] {
        let vertical = self.vertical_split(area, &[60, 40]);
        let top = self.horizontal_split(vertical[0], &[60, 40]);
        let bottom = self.horizontal_split(vertical[1], &[50, 50]);

        [top[0], top[1], bottom[0], bottom[1]]
    }
}

impl Default for LayoutManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_manager_creation() {
        let manager = LayoutManager::new();
        assert!(std::mem::size_of_val(&manager) == 0);
    }

    #[test]
    fn test_default_trait() {
        let manager = LayoutManager::default();
        assert!(std::mem::size_of_val(&manager) == 0);
    }

    #[test]
    fn test_vertical_split() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 100, 100);
        let splits = manager.vertical_split(area, &[50, 50]);

        assert_eq!(splits.len(), 2);
        assert_eq!(splits[0].height + splits[1].height, area.height);
    }

    #[test]
    fn test_horizontal_split() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 100, 100);
        let splits = manager.horizontal_split(area, &[60, 40]);

        assert_eq!(splits.len(), 2);
        assert_eq!(splits[0].width + splits[1].width, area.width);
    }

    #[test]
    fn test_default_layout() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 100, 100);
        let panes = manager.default_layout(area);

        // Should return 4 panes
        assert_eq!(panes.len(), 4);

        // All panes should be within the original area
        for pane in &panes {
            assert!(pane.x < area.width);
            assert!(pane.y < area.height);
            assert!(pane.width > 0);
            assert!(pane.height > 0);
        }
    }

    #[test]
    fn test_uneven_splits() {
        let manager = LayoutManager::new();
        let area = Rect::new(0, 0, 100, 100);
        let splits = manager.vertical_split(area, &[30, 70]);

        assert_eq!(splits.len(), 2);
        // First split should be roughly 30% of height
        assert!(splits[0].height >= 28 && splits[0].height <= 32);
    }
}
