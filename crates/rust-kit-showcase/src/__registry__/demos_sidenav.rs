use std::sync::Arc;

use app_domain::markdown_config::registry_entry::RegistryEntry;

// * This file was generated automatically with build_registry.

pub fn get_all_arc_demos_for_components() -> Vec<(&'static str, Arc<Vec<RegistryEntry>>)> {
    vec![("Components", Arc::new(ALL_SIDENAV_COMPONENTS.to_vec()))]
}

pub fn get_all_arc_demos_for_hooks() -> Vec<(&'static str, Arc<Vec<RegistryEntry>>)> {
    vec![]
}

pub const ALL_SIDENAV_COMPONENTS: &[RegistryEntry] = &[RegistryEntry {
    title: "Button",
    path_url: "button",
    path_md: "public/docs/components/button.md",
    description: "Rust/UI component that displays a button or a component that looks like a button.",
    tags: &["button"],
    image: "/images/thumbnails/button.webp",
    image_dark: "/images/thumbnails/button-dark.webp",
    is_new: false,
    order: None,
}];

pub const ALL_SIDENAV_HOOKS: &[RegistryEntry] = &[];
