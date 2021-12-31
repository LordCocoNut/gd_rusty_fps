# gd_rusty_fps
FPS library for gdnative written in Rust.

This projects aims to create easy to use .dll library to be used with godot engine for FPS game development.

<hr>
We have to provide these features:

- [x] camera look controller -> basic component is done
- [x] player body walking with slopes etc fixes -> basic component is done, requires jumping that will be implemented later on
- [ ] game hud stuff
- [x] world interactor -> player should be able to interact with mobs
        -> the interaction in global is implemented with possibility to pickup items. We gonna 
            add more functionalities on the go as we build a game with this library
- [ ] implement multilang labels system, to allow game multilanguage
- [ ] implement quests system, that is aware of actions, picking up items
- [ ] implement player action system (we have to be able to provide funcionalities that is able to provide reactions to players actions).
        -> when player gets to area, waits for some time etc. It should be also possible to be connected to quests system aswell.
- [ ] implement jumping for player controller
<hr>
<b> this library should then be used to create a simple horror adventure game for demonstration.<b>

DOC
<hr>
Interactible types:
    - [x] Pickupable
        items that can be added to inventory on pickup
    - [ ] Switch
        item that triggers run action on 
    - [ ] Openable
