import os
import re

workspace = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

errors = []

def find_matching_div(content, start_idx):
    idx = start_idx
    depth = 0
    while idx < len(content):
        if content[idx:].startswith('<div') or content[idx:].startswith('<DIV'):
            m = re.match(r'^<div[\s>]', content[idx:], re.IGNORECASE)
            if m:
                depth += 1
                idx += 4
                continue
        elif content[idx:].startswith('</div>') or content[idx:].startswith('</DIV>'):
            depth -= 1
            idx += 6
            if depth == 0:
                return idx
            continue
        idx += 1
    return -1

# Validate HTML blocks
print("Validating HTML blocks in notes.html files...")
for root, dirs, files in os.walk(workspace):
    for f in files:
        if f == "notes.html":
            notes_path = os.path.join(root, f)
            with open(notes_path, 'r', encoding='utf-8') as fh:
                html_content = fh.read()
            
            pos = 0
            while True:
                match = re.search(r'<div\s+class="codeblock"[^>]*data-cb', html_content[pos:], re.IGNORECASE)
                if not match:
                    break
                
                start_idx = pos + match.start()
                end_idx = find_matching_div(html_content, start_idx)
                
                if end_idx == -1:
                    errors.append(f"In {notes_path}: Unmatched div at index {start_idx}")
                    pos = start_idx + 20
                    continue
                
                block_text = html_content[start_idx:end_idx]
                pos = end_idx
                
                buttons = re.findall(r'data-lang="([^"]+)"', block_text)
                panels = re.findall(r'data-panel="([^"]+)"', block_text)
                
                # Check that for every button there is a panel
                for btn in buttons:
                    if btn not in panels:
                        errors.append(f"In {notes_path}: tab button with lang '{btn}' has no matching panel inside the codeblock starting at {start_idx}")
                    else:
                        if f'data-panel="{btn}"' not in block_text:
                            errors.append(f"In {notes_path}: panel for '{btn}' not found correctly.")

# Validate physical snippet files
print("Validating physical snippet files...")
for root, dirs, files in os.walk(workspace):
    if root.endswith("code/go") or root.endswith("code/python"):
        parent_dir = os.path.dirname(root)
        ts_dir = os.path.join(parent_dir, "typescript")
        rs_dir = os.path.join(parent_dir, "rust")
        
        is_go = root.endswith("go")
        
        for f in files:
            base, ext = os.path.splitext(f)
            if (is_go and ext == ".go") or (not is_go and ext == ".py"):
                ts_file = os.path.join(ts_dir, base + ".ts")
                rs_file = os.path.join(rs_dir, base + ".rs")
                
                if not os.path.exists(ts_file):
                    errors.append(f"Missing matching TS snippet file: {ts_file}")
                if not os.path.exists(rs_file):
                    errors.append(f"Missing matching Rust snippet file: {rs_file}")

if errors:
    print(f"Validation FAILED with {len(errors)} errors:")
    for err in errors[:20]:
        print(f" - {err}")
    if len(errors) > 20:
        print(f"... and {len(errors) - 20} more errors.")
    exit(1)
else:
    print("Validation PASSED! All tab buttons have matching panels and all snippet files exist.")
    exit(0)
