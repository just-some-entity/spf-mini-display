pub enum Action
{
    Update,
    PlayOrResume,
    Next,
    Previous,
    VolumeUp(u8),
    VolumeDown(u8),
    ToggleRepeat,
    ToggleShuffle
}