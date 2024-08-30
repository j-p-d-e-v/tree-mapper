import os

def create_nested_directories(base_dir, depth):
    current_dir = base_dir
    
    for level in range(1, depth + 1):
        current_dir = os.path.join(current_dir, f"level_{level}")
        os.makedirs(current_dir, exist_ok=True)
        
        # Create a file at each level
        with open(os.path.join(current_dir, f"file_level_{level}.txt"), "w") as file:
            file.write(f"This is level {level}\n")
    
    print(f"Directory structure with depth {depth} created successfully under {base_dir}.")

# Define the depth (at least 6 levels deep)
depth = 6

# Create the nested directories
create_nested_directories("test-data", 6)