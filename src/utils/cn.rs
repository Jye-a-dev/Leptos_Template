use tw_merge::merge::tw_merge_slice;

/// Merges Tailwind CSS class strings cleanly while resolving conflicts.
pub fn cn(classes: &[&str]) -> String {
  tw_merge_slice(classes)
}
