# gd_rusty_fps
FPS library for gdnative written in Rust.

This projects aims to create easy to use .dll library to be used with godot engine for FPS game development.

<hr />
We have to provide these features:

- [x] camera look controller -> basic component is done
- [x] player body walking with slopes etc fixes -> basic component is done, requires jumping that will be implemented later on
- [ ] game hud stuff
- [x] world interactor -> player should be able to interact with mobs
        -> the interaction in global is implemented with possibility to pickup items. We gonna 
            add more functionalities on the go as we build a game with this library
- [ ] implement multilang labels system, to allow game multilanguage
- [ ] implement supportive systems initialy with flashlight functionality
- [ ] implement quests system, that is aware of actions, picking up items
- [ ] implement player action system (we have to be able to provide funcionalities that is able to provide reactions to players actions).
        -> when player gets to area, waits for some time etc. It should be also possible to be connected to quests system aswell.
- [ ] implement jumping for player controller
<hr />
<b> this library should then be used to create a simple horror adventure game for demonstration.</b>


documentation
<hr />
Interactible types:
    - [x] Pickupable
        items that can be added to inventory on pickup
    - [ ] Switch
        item that triggers run action on 
    - [ ] Openable


Usage
<hr />
This library is ready for technical demos. Download actual release, or build the master branch yourself, by runing <b>cargo build</b> either for production or develop. Provide amundsen.dll to godot project and create gdnative library out of it. Then you can assign provided functionalities to your project.

PlayerController, PlayerCamera <br/>
Create KinematicBody scene and add Spatial CameraRotationHelper as child and Camera as child of CameraRotationHelper. Add gdnative script with PlayerController class name and script class from created gdnative library. Same goes for PlayerCamera that you should connect to Camera. Congrats. You should be now possessing a playable character, that can walk and move camera with mouse.
note: You have to also create move_forward, move_backward, move_left, move_right as key inputs in your project settings.

PlayerCameraFocus <br/>
Create RayCast as child of Camera in your character scene and name it Focus. Add this class as gdnative in same manner as with PlayerController, PlayerCamera. You are now posssessing functionality to interact with world.

Interactibles -> no providing Door, Pickupable<br/>
Simply create scene with a static body and provide one of these native classes from our library. You then have acces to exported properties from within the editor, so you can define basics like name, inspec labels etc. Every type has its own attributes and interactions are connected to PlayerCameraFocus, from which these events are called (open, close etc). Later on, we will add more of interactibles and also provide stuff like lock possibility for doors. Doors will implement attributes (key_id, switch_id), which will be proccessed to allow door unlocking.

Inventory <br/>
Create Spatial with name Inventory as child of KinematicBody in your character scene. It should connect itself to raycast on picked_up_item signal so items can be added to inventory on pickup event.



