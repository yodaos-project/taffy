#[cfg(test)]
mod caching {
    use taffy::prelude::*;

    struct CountMeasure {
        count: usize,
    }
    impl CountMeasure {
        fn new() -> CountMeasure {
            CountMeasure { count: 0 }
        }
    }

    fn count_measure_function(
        known_dimensions: Size<Option<f32>>,
        _available_space: Size<AvailableSpace>,
        _node_id: NodeId,
        mut node_context: Option<&mut CountMeasure>,
        _style: &Style,
    ) -> Size<f32> {
        node_context.as_mut().unwrap().count += 1;
        Size { width: known_dimensions.width.unwrap_or(50.0), height: known_dimensions.height.unwrap_or(50.0) }
    }

    #[test]
    fn measure_count_flexbox() {
        let mut taffy: TaffyTree<CountMeasure> = TaffyTree::new();

        let leaf = taffy.new_leaf_with_context(Style::default(), CountMeasure::new()).unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, count_measure_function).unwrap();

        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 4);
    }

    #[test]
    #[cfg(feature = "grid")]
    fn measure_count_grid() {
        let mut taffy: TaffyTree<CountMeasure> = TaffyTree::new();

        let style = || Style { display: Display::Grid, ..Default::default() };
        let leaf = taffy.new_leaf_with_context(style(), CountMeasure::new()).unwrap();

        let mut node = taffy.new_with_children(Style::DEFAULT, &[leaf]).unwrap();
        for _ in 0..100 {
            node = taffy.new_with_children(Style::DEFAULT, &[node]).unwrap();
        }

        taffy.compute_layout_with_measure(node, Size::MAX_CONTENT, count_measure_function).unwrap();
        assert_eq!(taffy.get_node_context_mut(leaf).unwrap().count, 4);
    }

    #[test]
    #[cfg(feature = "block_layout")]
    fn block_relayout_updates_percent_max_height_when_parent_height_changes() {
        let mut taffy = TaffyTree::<()>::new();
        let leaf = taffy
            .new_leaf(Style {
                display: Display::Block,
                max_size: Size { width: auto(), height: percent(0.5) },
                ..Default::default()
            })
            .unwrap();
        let parent_style = |height| Style {
            display: Display::Block,
            size: Size { width: length(100.0), height },
            ..Default::default()
        };
        let parent = taffy.new_with_children(parent_style(length(100.0)), &[leaf]).unwrap();

        for (height, expected) in [(length(100.0), 50.0), (length(200.0), 80.0), (length(60.0), 30.0), (auto(), 80.0)] {
            taffy.set_style(parent, parent_style(height)).unwrap();
            taffy
                .compute_layout_with_measure(parent, Size::MAX_CONTENT, |_, _, _, _, _| Size {
                    width: 80.0,
                    height: 80.0,
                })
                .unwrap();
            assert_eq!(taffy.layout(leaf).unwrap().size.height, expected, "parent height {height:?}");
        }
    }
}
