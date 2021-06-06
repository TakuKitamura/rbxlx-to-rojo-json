# rbxlx-to-rojo-json
Convert Rbxlx files to Rojo's project JSON format.

### Dependency
- Rust

### Run
```sh
$ cargo run /tmp/test/test.rbxlx ./default.project.json
--level1--
/DataModel, childrenNumber: 38
--level2--
/DataModel/Workspace, childrenNumber: 4
/DataModel/SoundService, childrenNumber: 0
/DataModel/NonReplicatedCSGDictionaryService, childrenNumber: 0
/DataModel/CSGDictionaryService, childrenNumber: 0
/DataModel/Chat, childrenNumber: 0
/DataModel/Instance, childrenNumber: 0
/DataModel/Players, childrenNumber: 0
/DataModel/ReplicatedFirst, childrenNumber: 0
/DataModel/TweenService, childrenNumber: 0
/DataModel/PermissionsService, childrenNumber: 0
/DataModel/PlayerEmulatorService, childrenNumber: 0
/DataModel/StudioData, childrenNumber: 0
/DataModel/StarterPlayer, childrenNumber: 2
/DataModel/StarterPack, childrenNumber: 0
/DataModel/StarterGui, childrenNumber: 0
/DataModel/LocalizationService, childrenNumber: 0
/DataModel/Teleport Service, childrenNumber: 0
/DataModel/CollectionService, childrenNumber: 0
/DataModel/PhysicsService, childrenNumber: 0
/DataModel/Geometry, childrenNumber: 0
/DataModel/InsertService, childrenNumber: 1
/DataModel/GamePassService, childrenNumber: 0
/DataModel/Debris, childrenNumber: 0
/DataModel/CookiesService, childrenNumber: 0
/DataModel/VRService, childrenNumber: 0
/DataModel/ContextActionService, childrenNumber: 0
/DataModel/Instance, childrenNumber: 0
/DataModel/AssetService, childrenNumber: 0
/DataModel/TouchInputService, childrenNumber: 0
/DataModel/AnalyticsService, childrenNumber: 0
/DataModel/Selection, childrenNumber: 0
/DataModel/ServerScriptService, childrenNumber: 1
/DataModel/ServerStorage, childrenNumber: 0
/DataModel/ReplicatedStorage, childrenNumber: 2
/DataModel/Instance, childrenNumber: 0
/DataModel/Lighting, childrenNumber: 0
/DataModel/HttpService, childrenNumber: 0
/DataModel/LanguageService, childrenNumber: 0
--level3--
/DataModel/Workspace/Camera, childrenNumber: 0
/DataModel/Workspace/BaseField, childrenNumber: 0
/DataModel/Workspace/SpawnLocation, childrenNumber: 1
/DataModel/Workspace/Terrain, childrenNumber: 0
/DataModel/StarterPlayer/StarterCharacterScripts, childrenNumber: 0
/DataModel/StarterPlayer/StarterPlayerScripts, childrenNumber: 1
/DataModel/InsertService/InsertionHash, childrenNumber: 0
/DataModel/ServerScriptService/TS, childrenNumber: 1
/DataModel/ReplicatedStorage/TS, childrenNumber: 1
/DataModel/ReplicatedStorage/rbxts_include, childrenNumber: 3
--level4--
/DataModel/Workspace/SpawnLocation/Decal, childrenNumber: 0
/DataModel/StarterPlayer/StarterPlayerScripts/TS, childrenNumber: 1
/DataModel/StarterPlayer/StarterPlayerScripts/TS/main, childrenNumber: 0
/DataModel/StarterPlayer/StarterPlayerScripts/TS/module, childrenNumber: 0
/DataModel/ReplicatedStorage/rbxts_include/Promise, childrenNumber: 0
/DataModel/ReplicatedStorage/rbxts_include/RuntimeLib, childrenNumber: 0
/DataModel/ReplicatedStorage/rbxts_include/node_modules, childrenNumber: 2
--level5--
/DataModel/StarterPlayer/StarterPlayerScripts/TS/main, childrenNumber: 0
/DataModel/ReplicatedStorage/rbxts_include/node_modules/compiler-types, childrenNumber: 1
/DataModel/ReplicatedStorage/rbxts_include/node_modules/types, childrenNumber: 1
--level6--
/DataModel/ReplicatedStorage/rbxts_include/node_modules/compiler-types/types, childrenNumber: 0
/DataModel/ReplicatedStorage/rbxts_include/node_modules/types/include, childrenNumber: 1
--level7--
/DataModel/ReplicatedStorage/rbxts_include/node_modules/types/include/generated, childrenNumber: 0
created ./default.project.json !
```