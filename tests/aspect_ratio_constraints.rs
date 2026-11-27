#[cfg(all(feature = "taffy_tree", any(feature = "block_layout", feature = "flexbox", feature = "grid")))]
mod tests {
    use taffy::prelude::*;
    use taffy::TaffyTree;

    #[test]
    fn leaf_respects_explicit_size_constraints_with_an_aspect_ratio() {
        for (size, min_size, max_size, expected) in [
            (
                Size { width: length(80.0), height: auto() },
                Size::AUTO,
                Size { width: auto(), height: length(50.0) },
                (80.0, 50.0),
            ),
            (
                Size { width: auto(), height: length(60.0) },
                Size { width: length(100.0), height: auto() },
                Size::AUTO,
                (100.0, 60.0),
            ),
        ] {
            let mut tree = TaffyTree::<()>::new();
            let child = tree
                .new_leaf(Style { size, min_size, max_size, aspect_ratio: Some(1.0), ..Default::default() })
                .unwrap();
            tree.compute_layout(child, Size::MAX_CONTENT).unwrap();
            let actual = tree.layout(child).unwrap().size;
            assert_eq!((actual.width, actual.height), expected);
        }
    }

    fn assert_percentage_size_with_cross_axis_constraint(display: Display) {
        for (size, max_size, expected) in [
            (Size { width: percent(1.0), height: auto() }, Size { width: auto(), height: length(40.0) }, (120.0, 40.0)),
            (Size { width: auto(), height: percent(1.0) }, Size { width: length(40.0), height: auto() }, (40.0, 240.0)),
        ] {
            let mut tree = TaffyTree::<()>::new();
            let child = tree.new_leaf(Style { size, max_size, aspect_ratio: Some(2.0), ..Default::default() }).unwrap();
            let parent = tree
                .new_with_children(
                    Style {
                        display,
                        size: Size { width: length(120.0), height: length(240.0) },
                        align_items: Some(AlignItems::Start),
                        ..Default::default()
                    },
                    &[child],
                )
                .unwrap();
            tree.compute_layout(parent, Size::MAX_CONTENT).unwrap();
            let actual = tree.layout(child).unwrap().size;
            assert_eq!((actual.width, actual.height), expected, "{display:?}");
        }
    }

    #[cfg(feature = "flexbox")]
    #[test]
    fn flexbox_preserves_the_resolved_percentage_size() {
        assert_percentage_size_with_cross_axis_constraint(Display::Flex);
    }

    #[cfg(feature = "grid")]
    #[test]
    fn grid_preserves_the_resolved_percentage_size() {
        assert_percentage_size_with_cross_axis_constraint(Display::Grid);
    }

    #[cfg(feature = "block_layout")]
    #[test]
    fn block_layout_preserves_the_resolved_percentage_size() {
        assert_percentage_size_with_cross_axis_constraint(Display::Block);
    }
}
