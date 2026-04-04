import os

def check_large_files(base_path, min_size_gb=10):
    min_bytes = min_size_gb * 1024**3
    base_path = os.path.expanduser(base_path)
        
    for root, dirs, files in os.walk(base_path):
        dirs[:] = [d for d in dirs if d not in [
            'node_modules', 'target', 'build', '.nvm', '.rustup', '.cargo',
            '.npm', 'venv', 'env'
        ]]
        
        for file in files:
            filepath = os.path.join(root, file)
            try:
                if os.path.islink(filepath): continue
                if not os.path.isfile(filepath): continue

                size = os.path.getsize(filepath)
                if size >= min_bytes:
                    print(f"FOUND: {filepath} ({size / 1024**3:.2f} GB)")
            except Exception as e:
                pass

check_large_files('~/')
