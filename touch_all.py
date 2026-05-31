import os

binary_exts = {'.bin', '.exe', '.dll', '.so', '.dylib', '.png', '.jpg'}
binary_files = {'test_dl'}

for root, dirs, files in os.walk('.'):
    if '.git' in dirs:
        dirs.remove('.git')
    if 'target' in dirs:
        dirs.remove('target')

    for file in files:
        if file in binary_files:
            continue
        ext = os.path.splitext(file)[1]
        if ext in binary_exts:
            continue
        
        filepath = os.path.join(root, file)
        try:
            with open(filepath, 'a', encoding='utf-8') as f:
                f.write('\n')
        except UnicodeDecodeError:
            pass


