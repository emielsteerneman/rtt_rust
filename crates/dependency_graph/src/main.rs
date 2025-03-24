#[allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};
use syn::TypePath;
use syn::{Field, GenericArgument, Item, PathArguments, Type};
use syn::{ImplItem, ItemUse, UseTree};

fn find_rust_files(base_dir: &Path) -> Vec<PathBuf> {
    // println!("[find_rust_files] {}", base_dir.to_str().unwrap());

    let mut rust_files = Vec::new();

    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                rust_files.extend(find_rust_files(&path))
            }
            if path.is_file() {
                let ext = path.extension();
                if ext.is_some() && ext.unwrap() == "rs" {
                    // println!(" !   {}", path.to_string_lossy());
                    rust_files.push(path);
                }
            }
        }
    }

    rust_files
}

fn recursive_usetree(tree: &UseTree, names_only: bool) -> Vec<String> {
    match tree {
        UseTree::Path(p) => {
            let at: String = p.ident.to_string();
            let others: Vec<String> = recursive_usetree(&p.tree, names_only);
            match names_only {
                true => others,
                false => others.iter().map(|o| at.clone() + "::" + o).collect(),
            }
        }
        UseTree::Name(n) => vec![n.ident.to_string()],
        UseTree::Glob(_) => vec!["*".to_string()],
        UseTree::Group(g) => g
            .items
            .iter()
            .map(|i| recursive_usetree(i, names_only))
            .flatten()
            .collect(),
        _ => vec![],
    }
}

fn make_graph(dependencies: &HashMap<String, Vec<String>>) {
    let mut my_graph: String = "".to_string();
    let mut my_nodes: Vec<String> = dependencies.keys().cloned().collect();
    let mut my_edges: Vec<(usize, usize)> = Vec::new();

    for (struct_name, struct_dependencies) in dependencies.iter() {
        if !my_nodes.contains(struct_name) {
            my_nodes.push(struct_name.clone());
        }
        let idx = my_nodes.iter().position(|x| x == struct_name).unwrap();
        for dep in struct_dependencies {
            if !my_nodes.contains(dep) {
                my_nodes.push(dep.clone());
            }
            let dep_idx = my_nodes.iter().position(|x| x == dep).unwrap();
            my_edges.push((idx, dep_idx));
        }
    }

    for (inode, node) in my_nodes.iter().enumerate() {
        my_graph += &format!("    {} [label=\"{}\"];\n", inode, node);
    }
    for (src, dst) in my_edges {
        my_graph += &format!("    {} -> {};\n", src, dst);
    }

    println!("digraph G {{\n{}}}\n", my_graph);
}

fn traverse_path(type_path: &TypePath) -> Vec<String> {
    // println!("\n[traverse_path]");
    let mut names: Vec<String> = Vec::new();
    let mut names_other: Vec<Vec<String>> = Vec::new();

    for path_segment in type_path.path.segments.iter() {
        let name: String = path_segment.ident.to_string();
        // println!("[traverse_path] path_segment {}", &name);
        names.push(name);

        if let PathArguments::AngleBracketed(idk) = &path_segment.arguments {
            // println!("[traverse_path] <{}>", idk.args.len());
            for arg in idk.args.iter() {
                if let GenericArgument::Type(Type::Path(t)) = arg {
                    // println!("[traverse_path] Found another TypePath");
                    names_other.push(traverse_path(&t));
                }
            }
        }
    }

    let mut names_other: Vec<String> = names_other.iter().flatten().cloned().collect();
    names.append(&mut names_other);
    names
}

fn main() {
    println!("Hello, world!");

    let rust_files = find_rust_files(Path::new("./crates"));

    for path in &rust_files {
        println!("  {}", path.to_str().unwrap());
    }

    println!("Found {} rust files", rust_files.len());

    let rust_contents: Vec<String> = rust_files
        .iter()
        .map(|path| fs::read_to_string(path).unwrap())
        .collect();

    let struct_types_ignore: HashSet<String> = [
        "bool",
        "HashMap",
        "f32",
        "u32",
        "u8",
        "usize",
        "Instant",
        "RefCell",
        "Option",
        "std",
        "String",
        "time",
        "Vec",
        "SMatrix",
        "SVector",
        "Vector2",
        "UdpSocket",
        "S",
        "O",
    ]
    .iter()
    .map(|&s| s.into())
    .collect();

    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
    let mut all_types: HashSet<String> = HashSet::new();

    for content in &rust_contents {
        let ast = syn::parse_file(&content).expect("Failed to parse file");

        for item in ast.items {
            /* if let Item::Use(use_item) = item {
                let uses: Vec<String> = recursive_usetree(&use_item.tree, false);

                for u in uses.iter() {
                    println!(" use {}", u)
                }
            } else */
            /*if let Item::Impl(impl_item) = item {
                if let syn::Type::Path(type_path) = &*impl_item.self_ty {
                    println!("    impl {}", type_path.path.segments.last().unwrap().ident);
                }

                // Find all function implementations
                for impl_item in impl_item.items {
                    if let ImplItem::Fn(method) = impl_item {
                        let fn_name = method.sig.ident.to_string();
                        println!("    () {}", fn_name);
                    }
                }
            } else */
            if let Item::Enum(enum_item) = item {
                println!("Found enum {}", enum_item.ident.to_string());
                dependencies.insert(enum_item.ident.to_string(), vec![]);
            } else if let Item::Struct(struct_item) = item {
                let struct_name = struct_item.ident.to_string();
                println!("Found struct {}", struct_name);

                let mut struct_types: HashSet<String> = HashSet::<String>::new();

                for field in struct_item.fields {
                    match &field.ty {
                        Type::Path(type_path) => {
                            let names: Vec<String> = traverse_path(&type_path);
                            let names: HashSet<String> = names.into_iter().collect();
                            struct_types.extend(names);
                        }
                        _ => todo!(),
                    }
                }
                let struct_types: Vec<String> = struct_types
                    .difference(&struct_types_ignore)
                    .cloned()
                    .collect();
                if 0 < struct_types.len() {
                    println!("    : {:?}", &struct_types);
                }

                all_types.extend(struct_types.iter().cloned());

                // Exclude blacklisted types and store the rest
                dependencies.insert(struct_name, struct_types);
            }
        }
    }

    make_graph(&dependencies);
}
