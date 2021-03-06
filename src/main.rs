use regex::Regex;
use serde_json::json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn get_original_access_path(custom_path: std::string::String) -> std::string::String {
    let splited = custom_path.split("/");

    let mut ret = "".to_string();

    for v in splited {
        if v.len() > 0 {
            ret += "/";
            let x: Vec<&str> = v.split("@@@").collect();
            let y = x[0].to_string();
            ret += &y;
        }
    }

    return ret;
}

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
        for (property_name, property_value_variant) in &child_instance.properties {
            let type_name = json!(property_value_variant.ty())
                .as_str()
                .unwrap()
                .to_string();
            if let Ok(_) = serde_json::to_string(&property_value_variant) {
                let mut property_value = json!(null);

                let mut find_known_property = false;

                match property_value_variant {
                    rbx_types::Variant::Axes(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::BinaryString(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Bool(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::BrickColor(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::CFrame(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                        let position = &mut property_value["Position"].as_array().unwrap().to_vec();
                        let orientation =
                            property_value["Orientation"].as_array().unwrap().to_vec();

                        let flat_orientation = &mut vec![];
                        for elem in orientation {
                            let vecter3 = elem.as_array().unwrap().to_vec();
                            for f in vecter3 {
                                flat_orientation.push(f);
                            }
                        }

                        position.append(flat_orientation);

                        property_value = json!(position);
                    }
                    rbx_types::Variant::Color3(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Color3uint8(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::ColorSequence(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Content(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Enum(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Faces(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Float32(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Float64(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Int32(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Int64(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::NumberRange(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::NumberSequence(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::PhysicalProperties(value) => {
                        if json!(value) != json!("Default") {
                            find_known_property = true;
                            property_value = json!(value);
                        }
                    }
                    rbx_types::Variant::Ray(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Rect(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Ref(value) => {
                        // skip
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Region3(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Region3int16(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::String(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::UDim(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::UDim2(value) => {
                        find_known_property = true;

                        property_value = json!(value);

                        let first = &mut property_value[0].as_array().unwrap().to_vec();
                        let last = &mut property_value[1].as_array().unwrap().to_vec();

                        first.append(last);

                        property_value = json!(first);
                    }
                    rbx_types::Variant::Vector2(value) => {
                        property_value = json!(value);
                        if property_value[0] != json!(null) && property_value[1] != json!(null) {
                            find_known_property = true;
                        }
                    }
                    rbx_types::Variant::Vector2int16(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Vector3(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::Vector3int16(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    rbx_types::Variant::OptionalCFrame(value) => {
                        find_known_property = true;
                        property_value = json!(value);
                    }
                    t => println!("Unkown Type: {:?}", t),
                }

                if find_known_property && property_value != json!(null) {
                    properties_map.insert(
                        property_name.to_string(),
                        json!({"Type": &type_name, "Value": property_value}),
                    );
                }
            }
        }

        instance_map[&*child_instance_ref]["$className"] = json!(child_instance.class);
        instance_map[&*child_instance_ref]["$properties"] = json!(properties_map);

        instance_map[&*child_instance_ref]["name"] = json!(format!(
            "{}@@@{}",
            child_instance.name,
            child_instance.referent()
        ));

        // instance_map[&*child_instance_ref]["originalName"] = json!(child_instance.name);
        instance_map[&*child_instance_ref]["children"] = json!(child_instance.children());

        // instance_map[&*child_instance_ref]["parentClass"] = json!(instance.class);

        instance_map[&*child_instance_ref]["parentName"] =
            json!(format!("{}@@@{}", instance.name, instance.referent()));

        // instance_map[&*child_instance_ref]["originalParentName"] = json!(instance.name);
        // instance_map[&*child_instance_ref]["parentRef"] = json!(instance.referent());

        instance_to_flat_json(tree, child_instance, instance_map);
    }
}

fn get_rbxlx_json(
    rbxlx_path: std::string::String,
) -> (
    serde_json::Value,
    std::collections::HashMap<std::string::String, std::string::String>,
) {
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

    flat_instances_json[&*root_instance_ref]["name"] = json!(format!(
        "{}@@@{}",
        root_instance.name,
        root_instance.referent()
    ));
    flat_instances_json[&*root_instance_ref]["children"] = json!(root_instance.children());
    // flat_instances_json[&*root_instance_ref]["parentClass"] = json!("");
    flat_instances_json[&*root_instance_ref]["parentName"] = json!("");
    // flat_instances_json[&*root_instance_ref]["parentRef"] = json!("");

    instance_to_flat_json(&tree, root_instance, &mut flat_instances_json);

    // println!("{:?}", flat_instances_json);

    let mut tmp_refs_array = Vec::new();

    let mut root_instance_name = &json!("");

    for (child_ref, instance_item) in &flat_instances_json {
        if instance_item["parentName"] == json!("") {
            root_instance_name = instance_item.get("name").unwrap();
            tmp_refs_array.push(vec![child_ref.to_string()]);
        }
    }

    let mut path_dict: HashMap<String, String> = HashMap::new();

    path_dict.insert(
        "/globIgnorePaths".to_string(),
        "/globIgnorePaths".to_string(),
    );
    path_dict.insert("/name".to_string(), "/name".to_string());

    let root_instance_original_name = get_original_access_path(
        "/".to_string() + &root_instance_name.as_str().unwrap().to_string(),
    );

    // println!("777: {}", root_instance_original_name);
    path_dict.insert(
        root_instance_original_name,
        root_instance_name.as_str().unwrap().to_string(),
    );

    let mut next_access_path: Vec<String> = Vec::new();

    let mut access_path_map: HashMap<String, Vec<String>> = HashMap::new();
    access_path_map.insert(root_instance_name.to_string(), vec![]);

    let mut rbxlx_json = json!({});

    let mut all_path = vec![];

    while tmp_refs_array.len() > 0 {
        let mut refs_array = Vec::new();

        refs_array = tmp_refs_array.to_vec();
        tmp_refs_array = Vec::new();

        // println!("--level{:?}--", i);

        for refs in refs_array {
            for next_ref in refs {
                let instance_item = flat_instances_json.get(&next_ref).unwrap();

                // if plugin access denied class
                if instance_item["$className"] == "CSGDictionaryService"
                    || instance_item["$className"] == "NonReplicatedCSGDictionaryService"
                {
                    continue;
                }

                let mut parent_name = instance_item["parentName"].as_str().unwrap().to_string();
                let my_name = instance_item["name"].as_str().unwrap().to_string();

                if parent_name == "" {
                    parent_name = root_instance_name.to_string();
                }

                // println!("{:?}{:?}", access_path_map, parent_name);

                next_access_path = access_path_map[&parent_name].to_vec();
                next_access_path.append(&mut vec![my_name.to_string()]);

                let children_refs = instance_item.get("children").unwrap().as_array().unwrap();

                let now_path = format!("/{}", next_access_path.join("/"));

                if all_path.iter().any(|i| i == &now_path) {
                    next_access_path.pop();
                    next_access_path.append(&mut vec![format!(
                        "{}@@@{}",
                        my_name,
                        next_ref // all_path.iter().filter(|&n| *n == now_path).count() + 1
                    )]);
                }

                all_path.push(now_path);

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

                let clean_instance_item = json!({
                    "$className": instance_item["$className"],
                    "$properties": instance_item["$properties"],

                });

                // println!("{}", path_str);

                *rbxlx_json.pointer_mut(&path_str).unwrap() = clean_instance_item;

                if children_refs.len() > 0 {
                    let mut children = vec![];
                    for child_ref in children_refs {
                        let r: std::string::String = child_ref.as_str().unwrap().to_string();
                        children.push(r);
                    }

                    tmp_refs_array.push(children);

                    let access_path_map_key = &next_access_path[next_access_path.len() - 1];

                    // println!("next_access_path {:?}", next_access_path);

                    access_path_map
                        .insert(access_path_map_key.to_string(), next_access_path.to_vec());
                }
            }
        }
    }

    // println!("{:?}", all_path);

    // for path in all_path {
    //     println!("path: {}", path);
    // }

    for custom_path in &all_path {
        let original_path = get_original_access_path(custom_path.to_string());

        path_dict.insert(original_path.to_string(), custom_path.to_string());
    }

    // println!("{:?}", path_dict);

    return (rbxlx_json, path_dict);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("usage: rbxlx-to-rojo-json [rbxlx_path] [output_json_path]");
        std::process::exit(1);
    }

    let rbxlx_path = args[1].to_string();

    let rbxlx_json_output_path = args[2].to_string();

    let (mut rbxlx_json, path_dict) = get_rbxlx_json(rbxlx_path);

    // println!("{}, {}", rbxlx_json, &path_dict[&"/DataModel".to_string()]);

    let root_path = &path_dict[&"/DataModel".to_string()];

    let tree = rbxlx_json.pointer_mut(root_path).unwrap();

    let mut rojo_json = json!({
        "globIgnorePaths": [
            "**/package.json",
            "**/tsconfig.json"
        ],
        "name": "rbxlx-to-rojo-json-sample",
        "tree": tree
    });

    let config_file = fs::read_to_string("config.json").unwrap();

    let config_json: serde_json::Map<std::string::String, serde_json::Value> =
        serde_json::from_str(&config_file).unwrap();

    for elem in config_json {
        // /DataModel@@@c35b90713fc0caaf236f3df0b5d4452c
        let path = elem.0.replace("/tree", "/DataModel");
        let value = elem.1;

        if path_dict.contains_key(&path) {
            let custom_path = path_dict[&path].to_string().replace(root_path, "/tree");
            *rojo_json.pointer_mut(&custom_path).unwrap() = json!(value);
        } else {
            // path don't exis
            let mut splited: Vec<&str> = path.split("/").collect();
            let last_item = splited.pop().unwrap();
            let joined = splited.join("/");

            let base_path = path_dict[&joined].to_string().replace(root_path, "/tree");

            let base_path_pointer = rojo_json.pointer_mut(&base_path).unwrap();
            base_path_pointer[last_item] = json!(value)
        }
    }

    let json_str = serde_json::to_string_pretty(&rojo_json).unwrap();

    let re = Regex::new(r#""(.*@@@.*)""#).unwrap();

    // let mut tmp_refs_array = Vec::new();

    let mut check_duplicate: HashMap<String, i32> = HashMap::new();

    let result = re.replace_all(&json_str, |caps: &regex::Captures| {
        let match_str = &caps[1];
        let splited_vecter: Vec<&str> = match_str.split("@@@").collect();
        let name: &str = splited_vecter[0];

        if name == "TS" {
            let ret = format!("\"{}\"", name);
            return ret;
        }

        if !check_duplicate.contains_key(&name.to_string()) {
            check_duplicate.insert(name.to_string(), 1);
            let ret = format!("\"{}\"", name);
            return ret;
        } else {
            *check_duplicate.get_mut(&name.to_string()).unwrap() += 1;
            let ret = format!("\"{}{}\"", name, check_duplicate[&name.to_string()]);
            return ret;
        }
    });

    let mut file = File::create(&rbxlx_json_output_path).unwrap();
    file.write_all(result.as_bytes()).unwrap();
    println!("created {} !", &rbxlx_json_output_path);
}
