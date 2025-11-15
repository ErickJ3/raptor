#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    GoToFirst,
    GoToLast,
    EnterDirectory,
    GoBack,
    GoToParent,
    GoToRoot,
    GoHome,
    ToggleLabels,
    Select(usize),
    ClearSelection,
}
