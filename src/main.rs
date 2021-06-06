use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn instance_to_flat_json(
    tree: &rbx_dom_weak::WeakDom,
    instance: &rbx_dom_weak::Instance,
    instance_map: &mut serde_json::Map<std::string::String, serde_json::Value>,
) {
    for child_ref in instance.children() {
        let child_instance = match tree.get_by_ref(*child_ref) {
            Some(child_instance) => child_instance,
            None => panic!("can't get child_instance"),
        };

        let child_instance_ref = child_instance.referent().to_string();

        instance_map.insert(child_instance_ref.to_string(), json!({}));

        let properties_map = &mut serde_json::Map::new();
        for (type_name, property_value) in &child_instance.properties {
            if let Ok(_) = serde_json::to_string(&property_value) {
                if json!(property_value) == json!({ "Float32": null }) {
                    properties_map.insert(
                        type_name.to_string(),
                        json!({"Type": type_name, "Value": 0.0}),
                    );
                    continue;
                }

                properties_map.insert(
                    type_name.to_string(),
                    json!({"Type": type_name, "Value": property_value}),
                );
            }
        }

        instance_map[&*child_instance_ref]["$className"] = json!(child_instance.class);
        instance_map[&*child_instance_ref]["$properties"] = json!(properties_map);
        instance_map[&*child_instance_ref]["name"] = json!(child_instance.name);
        instance_map[&*child_instance_ref]["children"] = json!(child_instance.children());

        instance_map[&*child_instance_ref]["parentClass"] = json!(instance.class);
        instance_map[&*child_instance_ref]["parentName"] = json!(instance.name);
        instance_map[&*child_instance_ref]["parentRef"] = json!(instance.referent());

        instance_to_flat_json(tree, child_instance, instance_map);
    }
}

fn get_rbxlx_json(rbxlx_path: std::string::String) -> std::string::String {
    let model_file = match fs::read_to_string(rbxlx_path) {
        Ok(model_file) => model_file,
        Err(error) => panic!("problem opening the file: {:?}", error),
    };

    let tree = match rbx_xml::from_str_default(model_file) {
        Ok(tree) => tree,
        Err(error) => panic!("problem parsing rbx-xml: {:?}", error),
    };

    let root_instance = match tree.get_by_ref(tree.root_ref()) {
        Some(root_instance) => root_instance,
        None => panic!("can't get root-instance"),
    };

    let mut flat_instances_json = serde_json::Map::new();

    let root_instance_ref = &root_instance.referent().to_string();

    flat_instances_json.insert(root_instance_ref.to_string(), json!({}));

    flat_instances_json[&*root_instance_ref]["$className"] = json!(root_instance.class);
    flat_instances_json[&*root_instance_ref]["$properties"] = json!(root_instance.properties);

    flat_instances_json[&*root_instance_ref]["name"] = json!(root_instance.name);
    flat_instances_json[&*root_instance_ref]["children"] = json!(root_instance.children());
    flat_instances_json[&*root_instance_ref]["parentClass"] = json!("");
    flat_instances_json[&*root_instance_ref]["parentName"] = json!("");
    flat_instances_json[&*root_instance_ref]["parentRef"] = json!("");

    instance_to_flat_json(&tree, root_instance, &mut flat_instances_json);

    let mut tmp_refs_array = Vec::new();

    let mut root_instance_name = &json!("");

    for (child_ref, instance_item) in &flat_instances_json {
        if instance_item["parentName"] == json!("") {
            root_instance_name = instance_item.get("name").unwrap();
            tmp_refs_array.push(vec![child_ref.to_string()]);
        }
    }

    let mut next_access_path: Vec<String> = Vec::new();
    let mut i = 0;
    let mut access_path_map: HashMap<String, Vec<String>> = HashMap::new();
    access_path_map.insert(root_instance_name.to_string(), vec![]);

    let mut rbxlx_json = json!({});

    while tmp_refs_array.len() > 0 {
        let mut refs_array = Vec::new();

        refs_array = tmp_refs_array.to_vec();
        tmp_refs_array = Vec::new();

        i = i + 1;
        println!("--level{:?}--", i);

        for refs in refs_array {
            for next_ref in refs {
                let instance_item = flat_instances_json.get(&next_ref).unwrap();

                let mut parent_name = instance_item["parentName"].as_str().unwrap().to_string();
                let my_name = instance_item["name"].as_str().unwrap().to_string();

                if parent_name == "" {
                    parent_name = root_instance_name.to_string();
                }

                next_access_path = access_path_map[&parent_name].to_vec();
                next_access_path.append(&mut vec![my_name]);

                let children_refs = instance_item.get("children").unwrap().as_array().unwrap();

                println!(
                    "/{}, childrenNumber: {:?}",
                    next_access_path.join("/"),
                    children_refs.len()
                );

                let mut path_str = "".to_string();
                for access_path in &next_access_path {
                    path_str += &format!("/{}", access_path.to_string()).to_string();
                    if rbxlx_json.pointer(&path_str) == None {
                        if path_str.match_indices("/").count() == 1 {
                            rbxlx_json[access_path].take();
                        } else {
                            let base_path = path_str.rsplit_once("/").unwrap().0;
                            let rbxlx_json_pointer = rbxlx_json.pointer_mut(base_path).unwrap();
                            rbxlx_json_pointer[access_path].take();
                        }
                    }
                }

                *rbxlx_json.pointer_mut(&path_str).unwrap() = json!(instance_item);

                if children_refs.len() > 0 {
                    let mut children = vec![];
                    for child_ref in children_refs {
                        let r: std::string::String = child_ref.as_str().unwrap().to_string();
                        children.push(r);
                    }

                    tmp_refs_array.push(children);

                    let access_path_map_key = &next_access_path[next_access_path.len() - 1];

                    access_path_map
                        .insert(access_path_map_key.to_string(), next_access_path.to_vec());
                }
            }
        }
    }

    return serde_json::to_string(&rbxlx_json).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("usage: rbxlx-to-rojo-json [rbxlx_path] [output_json_path]");
        std::process::exit(1);
    }

    let rbxlx_path = args[1].to_string();

    let rbxlx_json_output_path = args[2].to_string();

    let rbxlx_json = get_rbxlx_json(rbxlx_path);

    let mut file = File::create(&rbxlx_json_output_path).unwrap();
    file.write_all(rbxlx_json.as_bytes()).unwrap();
    println!("created {} !", &rbxlx_json_output_path);
}