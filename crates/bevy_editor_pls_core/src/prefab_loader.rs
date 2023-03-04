use bevy::prelude::*;

pub struct PrefabLoaderPlugin;

//the intent here is that the user should be able to drag prefab .scn.ron files into the app and have them load into the scene.
impl Plugin for PrefabLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PrefabLoadEvent>()

            .add_system(file_drop)
            .add_system(add_prefab)
            ; 
    }
}

fn file_drop(
    mut dnd_evr: EventReader<FileDragAndDrop>,

    mut preload_evw: EventWriter<PrefabLoadEvent>,
) {
    for ev in dnd_evr.iter() {
        println!("{:?}", ev);
        if let FileDragAndDrop::DroppedFile { id, path_buf } = ev {
            println!("Dropped file with path: {:?}, in window id: {:?}", path_buf, id);
            
            preload_evw.send(
                PrefabLoadEvent {
                    //messy
                    path: path_buf.clone().into_os_string().into_string().unwrap(),
                }
            );
        }
    }
}

pub struct PrefabLoadEvent {
    pub path: String,
}

fn add_prefab(
    mut commands: Commands,
    server: Res<AssetServer>,

    mut preload_evr: EventReader<PrefabLoadEvent>,
)
{
    //just load to root, doesn't matter probably
    for ev in preload_evr.iter() {
        //can't find the DynamicSceneBundle type even though it's clearly defined in the prelude??
        // commands
        //     .spawn(
        //         DynamicSceneBundle {
        //             scene: server.load(ev.path.clone()),
        //             ..default()
        //         }
        //     );
    }
}
