# Hot Reload

Heavily inspired by <https://github.com/DGriffin91/ridiculous_bevy_hot_reloading>

Support:

- Reloadable components & resources via Reflect or Serde (within reason)
- Reloadable schedules - basically create a system that will run a schedule within another existing one if nothing has changed
- Redrawables - these basically would combine a state attached despawn & respawn situation - so you might have a `(setup_menu, cleanup_menu)` pair that gets triggered upon live reload
