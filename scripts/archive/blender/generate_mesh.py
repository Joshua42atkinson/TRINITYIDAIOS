import bpy
import sys
import json
import argparse

def main():
    if '--' in sys.argv:
        argv = sys.argv[sys.argv.index('--') + 1:]
    else:
        argv = []
        
    parser = argparse.ArgumentParser()
    parser.add_argument('--task', type=str, required=True, help='JSON string of the MeshTask')
    args = parser.parse_args(argv)
    
    task = json.loads(args.task)
    
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    
    mesh_type = task.get("mesh_type", "Cube").lower()
    scale = task.get("scale", [1.0, 1.0, 1.0])
    
    if mesh_type == "cube":
        bpy.ops.mesh.primitive_cube_add(scale=tuple(scale))
    elif mesh_type == "cylinder":
        bpy.ops.mesh.primitive_cylinder_add(scale=tuple(scale))
    elif mesh_type == "suzanne" or mesh_type == "monkey":
        bpy.ops.mesh.primitive_monkey_add(scale=tuple(scale))
    else:
        bpy.ops.mesh.primitive_cube_add(scale=tuple(scale))
        
    obj = bpy.context.active_object
    
    mat = bpy.data.materials.new(name="ComfyMaterial")
    mat.use_nodes = True
    bsdf = mat.node_tree.nodes.get("Principled BSDF")
    
    tex_image = mat.node_tree.nodes.new('ShaderNodeTexImage')
    try:
        tex_image.image = bpy.data.images.load(task["texture_path"])
        mat.node_tree.links.new(bsdf.inputs['Base Color'], tex_image.outputs['Color'])
    except Exception as e:
        print(f"Warning: Could not load texture from {task['texture_path']}: {e}")
        
    if obj.data.materials:
        obj.data.materials[0] = mat
    else:
        obj.data.materials.append(mat)
        
    bpy.ops.object.mode_set(mode='EDIT')
    bpy.ops.uv.smart_project()
    bpy.ops.object.mode_set(mode='OBJECT')
    
    output_path = task.get("output_path", "/tmp/output.glb")
    bpy.ops.export_scene.gltf(
        filepath=output_path,
        export_format='GLB',
        use_selection=True,
        export_materials='EXPORT'
    )
    
    print(f"Successfully generated and textured mesh at {output_path}")

if __name__ == "__main__":
    main()
