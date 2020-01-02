use crate::geometry::*;
use crate::scene::*;

use std::{fmt};
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct BVHInfo {
    primitive_number: usize,
    bounding_box: BoundingBox,
    center: [f32; 4],
    section: usize
}

impl BVHInfo {
    fn new(primitive_number: usize, bounding_box: BoundingBox) -> Self {
        let center = bounding_box.bounds[0] * 0.5 + bounding_box.bounds[1] * 0.5;

        Self {
            primitive_number: primitive_number,
            bounding_box: bounding_box,
            center: center.into(),
            section: 0
        }
    }
}

impl fmt::Display for BVHInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(primitive number: {}, bounding box: {})", self.primitive_number, self.bounding_box)
    }
}

pub struct BVHBuildNode {
    bounding_box: BoundingBox,
    children:[Box<Option<BVHBuildNode>>; 2],
    split_axis: i8,
    first_object_offset: u32,
    num_objects: u32
} 

impl BVHBuildNode {
    fn leaf_node(offset: u32, num_objects: u32, bounding_box: BoundingBox) -> Self {
        Self {
            bounding_box: bounding_box,
            children: [Box::new(None), Box::new(None)],
            split_axis: -1,
            first_object_offset: offset,
            num_objects: num_objects
        }
    }

    fn interior_node(axis: i8, child1:BVHBuildNode, child2:BVHBuildNode) -> Self {    
        Self {
            bounding_box: child1.bounding_box.union(child2.bounding_box),
            children: [Box::new(Some(child1)), Box::new(Some(child2))],
            split_axis: axis,
            first_object_offset: 0,
            num_objects: 0
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LinearBVHNode {
    pub bounding_box: BoundingBox,
    pub prim_offset: i32,
    pub second_child_offset: i32,
    pub num_prim: u32,
    pub axis: i8
}


impl LinearBVHNode {
    fn new(bounding_box: BoundingBox) -> Self {
        Self {
            bounding_box: bounding_box,
            prim_offset: -1,
            second_child_offset: -1,
            num_prim: 0,
            axis: -1
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Section {
    bounding_box: BoundingBox,
    count: u32
}

impl Default for Section {
    fn default() -> Section {
        Section {
            bounding_box: BoundingBox::new(),
            count: 0
        }
    }
}

pub fn build_bvh(scene_objects: &[SceneObject]) -> (Vec<LinearBVHNode>, Vec<usize>) {
    println!("Construct BVH: ");
    let now = Instant::now();
   
    let mut bvh_info_collection = Vec::new();

    for i in 0..scene_objects.len() {
        let bvh_info = BVHInfo::new(i, scene_objects[i].bounding_box);
        bvh_info_collection.push(bvh_info);
    }
    
    let mut total_nodes = 0;
    let mut ordered_scene_object = Vec::new();
    let root = recursive_build_nodes(& mut bvh_info_collection, 0, scene_objects.len(), &mut total_nodes, scene_objects, & mut ordered_scene_object);

    let mut nodes = vec![LinearBVHNode::new(BoundingBox::new()); total_nodes];
    flatten_bvh_tree(&root, &mut 0, &mut nodes);

    let end = now.elapsed().as_secs() as f64 + now.elapsed().subsec_nanos() as f64 * 1e-9;
    println!("Created BVH in: {}", end);
    println!("Nodes created: {}", nodes.len());

    (nodes, ordered_scene_object)
}

fn recursive_build_nodes(bvh_info: & mut [BVHInfo], start: usize, end: usize, total_nodes: &mut usize, scene_objects: &[SceneObject], ordered_scene_object: &mut Vec<usize>) -> BVHBuildNode {
    *total_nodes += 1;

    let node;
    let num_objects = end - start;

    let mut bounds = BoundingBox::new();
    for i in start..end {
        bounds = bounds.union(bvh_info[i].bounding_box);
    }

    if num_objects == 1 {
        let first_offset = ordered_scene_object.len() as u32;

        for i in start..end {
            let object_index = bvh_info[i].primitive_number;
            ordered_scene_object.push(object_index);
        }

        node = BVHBuildNode::leaf_node(first_offset, num_objects as u32, bounds);
        return node;
    } else {
        let mut center_bounds = BoundingBox::new();
        for i in start..end {
            center_bounds = center_bounds.union_from_vector(bvh_info[i].center.into());
        }

        let dim = center_bounds.maximum_extent();

        let mut mid = (start + end) / 2;

        let min: [f32; 4] =  center_bounds.bounds[0].into();  
        let max: [f32; 4] =  center_bounds.bounds[1].into(); 
        
        if min[dim as usize] == max[dim as usize] {
            let first_offset = ordered_scene_object.len() as u32;

            for i in start..end {
                let object_index = bvh_info[i].primitive_number;
                ordered_scene_object.push(object_index);
            }
    
            node = BVHBuildNode::leaf_node(first_offset, num_objects as u32, bounds);
            return node;
        } else {
            if num_objects <= 4 {
                //equal partitation
                bvh_info[start..end].sort_unstable_by(|a, b| a.center[dim as usize].partial_cmp(&b.center[dim as usize]).unwrap());
            } else {
                //SAH
                let num_sections = 12;
                let mut sections = vec![Section {..Default::default()}; num_sections];

                //divide in equal size sections
                for i in start..end {
                    let a: [f32; 4] = center_bounds.offset(bvh_info[i].center.into()).into();
                    let mut b = num_sections * a[dim as usize] as usize;

                    if b == num_sections {
                        b = num_sections - 1;
                    }

                    // println!("num tris: {}", scene_objects[bvh_info[i].primitive_number].mesh.num_tris);
                    bvh_info[i].section = b;
                    sections[b].count += scene_objects[bvh_info[i].primitive_number].mesh.num_tris;
                    sections[b].bounding_box = sections[b].bounding_box.union(bvh_info[i].bounding_box);
                }

                let mut cost = vec![0.0; num_sections - 1];

                //estimate cost 
                //TODO calculate cost based on object now it's 1
                for i in 0..(num_sections - 1) {
                    let mut b0 = BoundingBox::new();
                    let mut b1 = BoundingBox::new();

                    let mut count0 = 0;
                    let mut count1 = 0;

                    for j in 0..(i + 1) {
                        b0 = b0.union(sections[j].bounding_box);
                        count0 = sections[j].count;
                    }

                    for j in i..(num_sections - 1) {
                        b1 = b1.union(sections[j].bounding_box);
                        count1 = sections[j].count;
                    }

                    cost[i] = 0.125 + (count0 as f32 * b0.surface_area() + count1 as f32 * b1.surface_area()) / bounds.surface_area();
                }

                //find cheapest split
                let mut min_cost = cost[0];
                let mut min_cost_split_at = 0;
                for i in 1..(num_sections - 1) {
                    if cost[i] < min_cost {
                        min_cost = cost[i];
                        min_cost_split_at = i;
                    }
                }

                let mut leaf_cost = 0;

                for i in start..end {
                    leaf_cost += scene_objects[i].mesh.num_tris;
                }

                if num_objects > 225 || min_cost < leaf_cost as f32 {
                    bvh_info[start..end].sort_unstable_by(|a, b| a.section.cmp(&b.section));

                    for i in start..end {
                        if bvh_info[i].section <= min_cost_split_at {
                            mid = i;
                        }
                    }
                } else {
                    let first_offset = ordered_scene_object.len() as u32;

                    for i in start..end {
                        let object_index = bvh_info[i].primitive_number;
                        ordered_scene_object.push(object_index);
                    }

                    node = BVHBuildNode::leaf_node(first_offset, num_objects as u32, bounds);
                    return node;
                }
            }          
            
            node = BVHBuildNode::interior_node(
                    dim as i8,
                    recursive_build_nodes(bvh_info, start, mid, total_nodes, scene_objects, ordered_scene_object), 
                    recursive_build_nodes(bvh_info, mid, end, total_nodes, scene_objects, ordered_scene_object)
                );
            return node;
        }
    }
}

fn flatten_bvh_tree(build_node: &BVHBuildNode, offset: &mut usize, nodes:  &mut [LinearBVHNode]) -> u32 {
    let current_offset = *offset;

    nodes[current_offset].bounding_box = build_node.bounding_box;

    *offset += 1;
    // let new_offset = *offset;

    if build_node.num_objects > 0 {
        println!("BVH node objects: {}",  build_node.num_objects);
        nodes[current_offset].prim_offset = build_node.first_object_offset as i32;
        nodes[current_offset].num_prim = build_node.num_objects;
    } else {
        nodes[current_offset].axis = build_node.split_axis as i8;

        match &*build_node.children[0] {
            Some(node) => {
                flatten_bvh_tree(&node, offset, nodes);
            },
            None => ()
        }

        match &*build_node.children[1] {
            Some(node) => {
                let second_child_offset = flatten_bvh_tree(&node, offset, nodes) as i32;
                nodes[current_offset].second_child_offset = second_child_offset;
            },
            None => ()
        }   
    }


    current_offset as u32
}