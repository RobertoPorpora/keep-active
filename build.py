import os
import shutil
import subprocess
import platform
import time

# Determine the target operating system
target_os = platform.system().lower()

# Set the output directory based on the target OS
output_dir = f"dist/{target_os}"

# Clean Cargo project
# print("Cleaning Cargo project...")
# subprocess.run(["cargo", "clean"], check=True)

# Build Cargo project in release mode
print("Building Cargo project in release mode...")
subprocess.run(["cargo", "build", "--release"], check=True)

# Create output directory if it doesn't exist
if not os.path.exists(output_dir):
    print(f"Creating {output_dir} directory...")
    os.makedirs(output_dir)
else:
    # Delete contents of output directory
    print(f"Deleting contents of {output_dir} directory...")
    for filename in os.listdir(output_dir):
        file_path = os.path.join(output_dir, filename)
        try:
            if os.path.isfile(file_path) or os.path.islink(file_path):
                os.unlink(file_path)
            elif os.path.isdir(file_path):
                shutil.rmtree(file_path)
        except Exception as e:
            print(f"Failed to delete {file_path}. Reason: {e}")

# Copy compiled executable to output directory
executable_name = "keep-active.exe" if target_os == "windows" else "keep-active"
print(f"Copying compiled executable to {output_dir} directory...")
shutil.copy(f"target/release/{executable_name}", output_dir)

# Copy files and directories from include/ folder to output directory
# print(f"Copying files and directories from example/ folder to {output_dir} directory...")
# shutil.copytree("example", output_dir, dirs_exist_ok=True)

# Inform the user that build is complete
print("Build completed.")
