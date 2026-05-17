use dioxus::prelude::document::eval;
use dioxus::prelude::*;

#[component]
pub fn RichTextEditor(note_id: String, content: String, oninput: EventHandler<String>) -> Element {
    // We render markdown to HTML in Rust for the initial content
    let initial_html = crate::core::markdown::render_markdown(&content);

    // Track the last loaded note ID to prevent caret jumping
    let mut last_note_id = use_signal(String::new);

    // Sync note content when a new note ID is loaded
    let note_id_clone = note_id.clone();
    use_effect(move || {
        let current_id = note_id_clone.clone();
        if current_id != *last_note_id.read() {
            last_note_id.set(current_id.clone());
            let html_escaped = initial_html
                .replace('\\', "\\\\")
                .replace('\'', "\\'")
                .replace('\n', "\\n")
                .replace('\r', "\\r");
            let eval_str = format!(
                r#"
                let editor = document.getElementById('richtext-editor');
                if (editor) {{
                    editor.innerHTML = '{}';
                    if (window.enableCheckboxes) {{
                        window.enableCheckboxes();
                    }}
                    if (window.setupCodeCopyButtons) {{
                        window.setupCodeCopyButtons();
                    }}
                    if (window.updateToolbarState) {{
                        window.updateToolbarState();
                    }}
                }}
                "#,
                html_escaped
            );
            let _ = eval(&eval_str);
        }
    });

    // Initialize JS helpers and input event listener on mount
    use_effect(move || {
        let eval_str = r#"
            // Helper function to execute editing commands
            window.executeEditorCommand = function(cmd, val) {
                try {
                    document.execCommand('styleWithCSS', false, false);
                } catch (e) {}
                
                if (cmd === "insertTaskList") {
                    document.execCommand('insertHTML', false, '<ul class="task-list" style="list-style-type: none; padding-left: 0;"><li style="display: flex; align-items: center; gap: 8px;"><input type="checkbox" style="cursor: pointer;">&nbsp;Task item</li></ul>');
                    window.enableCheckboxes();
                    return;
                }
                
                document.execCommand(cmd, false, val);
                let editor = document.getElementById('richtext-editor');
                if (editor) {
                    if (window.setupCodeCopyButtons) {
                        window.setupCodeCopyButtons();
                    }
                    editor.dispatchEvent(new Event('input', { bubbles: true }));
                    if (window.updateToolbarState) {
                        window.updateToolbarState();
                    }
                }
            };

            // Enable checklists and bind changes to trigger input event
            window.enableCheckboxes = function() {
                let editor = document.getElementById('richtext-editor');
                if (!editor) return;
                let checkboxes = editor.querySelectorAll('input[type="checkbox"]');
                checkboxes.forEach(cb => {
                    cb.removeAttribute('disabled');
                    cb.style.cursor = 'pointer';
                    if (!cb.dataset.hasListener) {
                        cb.dataset.hasListener = "true";
                        cb.addEventListener('change', () => {
                            let editor = document.getElementById('richtext-editor');
                            if (editor) {
                                editor.dispatchEvent(new Event('input', { bubbles: true }));
                            }
                        });
                    }
                });
            };

            // Setup floating, premium copy buttons inside pre blocks
            window.setupCodeCopyButtons = function() {
                let editor = document.getElementById('richtext-editor');
                if (!editor) return;
                
                let pres = editor.querySelectorAll('pre');
                pres.forEach(pre => {
                    if (!pre.querySelector('.code-copy-btn')) {
                        let btn = document.createElement('button');
                        btn.className = 'code-copy-btn';
                        btn.setAttribute('contenteditable', 'false');
                        btn.textContent = 'Copy';
                        btn.addEventListener('click', (e) => {
                            e.preventDefault();
                            e.stopPropagation();
                            
                            // Get code text content by stripping the Copy button itself
                            let textToCopy = "";
                            pre.childNodes.forEach(child => {
                                if (child !== btn) {
                                    textToCopy += child.textContent;
                                }
                            });
                            
                            navigator.clipboard.writeText(textToCopy.trim()).then(() => {
                                btn.textContent = 'Copied!';
                                btn.style.color = 'var(--accent-green, #00e9a3)';
                                btn.style.borderColor = 'var(--accent-green, #00e9a3)';
                                setTimeout(() => {
                                    btn.textContent = 'Copy';
                                    btn.style.color = '';
                                    btn.style.borderColor = '';
                                }, 2000);
                            });
                        });
                        
                        // Prepend copy button inside pre
                        pre.insertBefore(btn, pre.firstChild);
                    }
                });
            };

            // Dynamic highlighting of toolbar options based on active cursor state
            window.updateToolbarState = function() {
                let editor = document.getElementById('richtext-editor');
                if (!editor) return;
                
                let buttons = document.querySelectorAll('.toolbar-btn');
                buttons.forEach(btn => {
                    let cmd = btn.getAttribute('title');
                    let val = btn.getAttribute('data-val');
                    let isActive = false;
                    
                    if (cmd === 'bold' || cmd === 'italic' || cmd === 'underline') {
                        isActive = document.queryCommandState(cmd);
                    } else if (cmd === 'insertUnorderedList') {
                        isActive = document.queryCommandState('insertUnorderedList');
                    } else if (cmd === 'insertOrderedList') {
                        isActive = document.queryCommandState('insertOrderedList');
                    } else if (cmd === 'insertTaskList') {
                        let selection = window.getSelection();
                        if (selection.rangeCount > 0) {
                            let parent = selection.getRangeAt(0).startContainer;
                            while (parent && parent !== editor) {
                                if (parent.tagName && parent.tagName.toLowerCase() === 'li') {
                                    if (parent.querySelector('input[type="checkbox"]')) {
                                        isActive = true;
                                        break;
                                    }
                                }
                                if (parent.classList && parent.classList.contains('task-list')) {
                                    isActive = true;
                                    break;
                                }
                                parent = parent.parentNode;
                            }
                        }
                    } else if (cmd === 'formatBlock' && val) {
                        let cleanTag = val.replace(/['<>]/g, '').toLowerCase();
                        let selection = window.getSelection();
                        if (selection.rangeCount > 0) {
                            let parent = selection.getRangeAt(0).startContainer;
                            while (parent && parent !== editor) {
                                if (parent.tagName && parent.tagName.toLowerCase() === cleanTag) {
                                    isActive = true;
                                    break;
                                }
                                parent = parent.parentNode;
                            }
                        }
                    }
                    
                    if (isActive) {
                        btn.classList.add('active');
                        btn.style.background = 'var(--bg-surface-high)';
                        btn.style.color = 'var(--accent)';
                    } else {
                        btn.classList.remove('active');
                        btn.style.background = 'transparent';
                        btn.style.color = 'var(--text-secondary)';
                    }
                });
            };

            // Recursive function to convert DOM node to clean Markdown
            window.toMarkdown = function(node) {
                if (node.nodeType === 3) {
                    return node.nodeValue;
                }
                if (node.nodeType !== 1) {
                    return "";
                }
                
                let tag = node.tagName.toLowerCase();
                
                if (tag === "pre") {
                    let codeText = "";
                    for (let child of node.childNodes) {
                        if (child.classList && child.classList.contains('code-copy-btn')) {
                            continue;
                        }
                        codeText += child.textContent;
                    }
                    return "\n```\n" + codeText.trim() + "\n```\n";
                }
                if (tag === "code") {
                    if (node.parentNode && node.parentNode.tagName.toLowerCase() !== "pre") {
                        return "`" + node.textContent + "`";
                    }
                    return node.textContent;
                }
                
                let children = "";
                for (let child of node.childNodes) {
                    children += window.toMarkdown(child);
                }
                
                switch (tag) {
                    case "h1":
                        return "\n# " + children.trim() + "\n";
                    case "h2":
                        return "\n## " + children.trim() + "\n";
                    case "h3":
                        return "\n### " + children.trim() + "\n";
                    case "strong":
                    case "b":
                        return children.trim() ? "**" + children.trim() + "**" : "";
                    case "em":
                    case "i":
                        return children.trim() ? "*" + children.trim() + "*" : "";
                    case "u":
                        return children.trim() ? "<u>" + children.trim() + "</u>" : "";
                    case "span":
                        let res = children;
                        let fw = node.style.fontWeight || "";
                        if (fw === "bold" || fw === "700" || fw === "800" || fw === "900") {
                            res = "**" + res.trim() + "**";
                        }
                        let fs = node.style.fontStyle || "";
                        if (fs === "italic") {
                            res = "*" + res.trim() + "*";
                        }
                        let td = node.style.textDecoration || "";
                        let tdl = node.style.textDecorationLine || "";
                        if (td.includes("underline") || tdl.includes("underline")) {
                            res = "<u>" + res.trim() + "</u>";
                        }
                        return res;
                    case "p":
                    case "div":
                        let text = children.trim();
                        return text ? "\n" + text + "\n" : "";
                    case "br":
                        return "\n";
                    case "ul":
                        return "\n" + children + "\n";
                    case "ol":
                        return "\n" + children + "\n";
                    case "li":
                        let hasCheckbox = node.querySelector('input[type="checkbox"]');
                        if (hasCheckbox) {
                            let isChecked = hasCheckbox.checked ? "x" : " ";
                            let textContent = "";
                            for (let child of node.childNodes) {
                                if (child.tagName && child.tagName.toLowerCase() === "input" && child.type === "checkbox") {
                                    continue;
                                }
                                textContent += window.toMarkdown(child);
                            }
                            return "\n- [" + isChecked + "] " + textContent.trim();
                        }
                        let parent = node.parentNode;
                        if (parent && parent.tagName.toLowerCase() === "ol") {
                            let index = Array.from(parent.children).indexOf(node) + 1;
                            return "\n" + index + ". " + children.trim();
                        }
                        return "\n- " + children.trim();
                    case "blockquote":
                        return "\n> " + children.trim() + "\n";
                    case "hr":
                        return "\n---\n";
                    default:
                        return children;
                }
            };

            // Main function to convert editor innerHTML to clean Markdown
            window.editorToMarkdown = function() {
                let editor = document.getElementById('richtext-editor');
                if (!editor) return "";
                let md = window.toMarkdown(editor);
                md = md.replace(/\n{3,}/g, '\n\n');
                return md.trim();
            };

            // Set up keyboard event listeners for active checklists on Enter
            let editor = document.getElementById('richtext-editor');
            if (editor) {
                try {
                    document.execCommand('styleWithCSS', false, false);
                } catch (e) {}
                
                // Keydown interceptor for checklist continuity, quote exit, and pre-formatted text
                editor.addEventListener('keydown', (e) => {
                    if (e.key === 'Enter') {
                        let selection = window.getSelection();
                        if (!selection.rangeCount) return;
                        let range = selection.getRangeAt(0);
                        let activeElement = range.startContainer;
                        
                        // 1. Code Block (pre) check
                        let pre = activeElement;
                        while (pre && pre !== editor && pre.tagName !== 'PRE') {
                            pre = pre.parentNode;
                        }
                        if (pre && pre.tagName === 'PRE') {
                            // If they press Ctrl+Enter (or Cmd+Enter), breakout of the code block!
                            if (e.ctrlKey || e.metaKey) {
                                e.preventDefault();
                                let text = pre.textContent;
                                
                                // Strip the trailing newline from the pre content if it exists
                                if (text.endsWith('\n')) {
                                    pre.textContent = text.slice(0, -1);
                                }
                                
                                // Create a new paragraph below the pre block
                                let p = document.createElement('p');
                                p.innerHTML = '<br>';
                                pre.parentNode.insertBefore(p, pre.nextSibling);
                                
                                // If the pre is completely empty, remove it to return to normal text!
                                if (pre.textContent.trim() === "") {
                                    pre.remove();
                                }
                                
                                // Position the cursor inside the new paragraph
                                let newRange = document.createRange();
                                newRange.setStart(p, 0);
                                newRange.collapse(true);
                                selection.removeAllRanges();
                                selection.addRange(newRange);
                                
                                editor.dispatchEvent(new Event('input', { bubbles: true }));
                                if (window.updateToolbarState) {
                                    window.updateToolbarState();
                                }
                                return;
                            } else {
                                // Any other Enter or Shift+Enter inside Code Block: ALWAYS insert literal newline character directly!
                                e.preventDefault();
                                let range = selection.getRangeAt(0);
                                range.deleteContents();
                                let textNode = document.createTextNode('\n');
                                range.insertNode(textNode);
                                
                                // Move cursor right after the newly inserted newline character
                                range.setStartAfter(textNode);
                                range.collapse(true);
                                selection.removeAllRanges();
                                selection.addRange(range);
                                
                                editor.dispatchEvent(new Event('input', { bubbles: true }));
                                return;
                            }
                        }
                        
                        // 2. Blockquote check
                        let bq = activeElement;
                        while (bq && bq !== editor && bq.tagName !== 'BLOCKQUOTE') {
                            bq = bq.parentNode;
                        }
                        if (bq && bq.tagName === 'BLOCKQUOTE') {
                            let textContent = activeElement.textContent.trim();
                            if (textContent === "") {
                                e.preventDefault();
                                
                                let p = document.createElement('p');
                                p.innerHTML = '<br>';
                                bq.parentNode.insertBefore(p, bq.nextSibling);
                                
                                if (bq.textContent.trim() === "") {
                                    bq.remove();
                                } else {
                                    let nodeToRemove = activeElement;
                                    while (nodeToRemove && nodeToRemove.parentNode !== bq) {
                                        nodeToRemove = nodeToRemove.parentNode;
                                    }
                                    if (nodeToRemove) {
                                        nodeToRemove.remove();
                                    }
                                }
                                
                                let newRange = document.createRange();
                                newRange.setStart(p, 0);
                                newRange.collapse(true);
                                selection.removeAllRanges();
                                selection.addRange(newRange);
                                
                                editor.dispatchEvent(new Event('input', { bubbles: true }));
                                if (window.updateToolbarState) {
                                    window.updateToolbarState();
                                }
                                return;
                            }
                        }
                        
                        // 3. Checklist LI check
                        let li = activeElement;
                        while (li && li !== editor && li.tagName !== 'LI') {
                            li = li.parentNode;
                        }
                        
                        if (li && li.tagName === 'LI') {
                            let hasCheckbox = li.querySelector('input[type="checkbox"]');
                            if (hasCheckbox) {
                                // If the current item's text content is empty, exit the checklist!
                                let textContent = li.textContent.trim();
                                if (textContent === "") {
                                    e.preventDefault();
                                    
                                    let p = document.createElement('p');
                                    p.innerHTML = '<br>';
                                    
                                    let parentList = li.parentNode;
                                    parentList.parentNode.insertBefore(p, parentList.nextSibling);
                                    li.remove();
                                    
                                    let newRange = document.createRange();
                                    newRange.setStart(p, 0);
                                    newRange.collapse(true);
                                    selection.removeAllRanges();
                                    selection.addRange(newRange);
                                    
                                    if (parentList.children.length === 0) {
                                        parentList.remove();
                                    }
                                    
                                    editor.dispatchEvent(new Event('input', { bubbles: true }));
                                    return;
                                }
                                
                                // Otherwise, let the browser enter and then format the next LI on the next tick
                                setTimeout(() => {
                                    let newSelection = window.getSelection();
                                    if (!newSelection.rangeCount) return;
                                    let newRange = newSelection.getRangeAt(0);
                                    let newActive = newRange.startContainer;
                                    let newLi = newActive;
                                    while (newLi && newLi !== editor && newLi.tagName !== 'LI') {
                                        newLi = newLi.parentNode;
                                    }
                                    if (newLi && newLi.tagName === 'LI' && !newLi.querySelector('input[type="checkbox"]')) {
                                        let cb = document.createElement('input');
                                        cb.type = 'checkbox';
                                        cb.style.cursor = 'pointer';
                                        newLi.insertBefore(cb, newLi.firstChild);
                                        window.enableCheckboxes();
                                        
                                        let cursorRange = document.createRange();
                                        cursorRange.setStartAfter(cb);
                                        cursorRange.collapse(true);
                                        newSelection.removeAllRanges();
                                        newSelection.addRange(cursorRange);
                                        editor.dispatchEvent(new Event('input', { bubbles: true }));
                                    }
                                }, 0);
                            }
                        }
                    }
                });

                if (!editor.dataset.listenerAttached) {
                    editor.dataset.listenerAttached = "true";
                    editor.addEventListener('input', () => {
                        window.enableCheckboxes();
                        if (window.setupCodeCopyButtons) {
                            window.setupCodeCopyButtons();
                        }
                        let md = window.editorToMarkdown();
                        dioxus.send(md);
                    });
                }
            }
            
            // Listen to selection changes to highlight active style indicators
            document.addEventListener('selectionchange', window.updateToolbarState);
        "#;

        let mut ev = eval(eval_str);

        // Listen to markdown updates sent from JS
        spawn(async move {
            while let Ok(val) = ev.recv::<serde_json::Value>().await {
                if let Some(md_str) = val.as_str() {
                    oninput.call(md_str.to_string());
                }
            }
        });
    });

    rsx! {
        style {
            r#"
            .editor-toolbar {{
                display: flex;
                align-items: center;
                gap: 4px;
                padding: 8px 16px;
                border-bottom: 1px solid var(--border);
                background: var(--bg-surface, #131313);
                flex-wrap: wrap;
                position: sticky;
                top: 0;
                z-index: 100;
                margin-bottom: 16px;
            }}
            .toolbar-btn {{
                padding: 6px 12px;
                border: none;
                border-radius: 6px;
                background: transparent;
                color: var(--text-secondary);
                cursor: pointer;
                font-family: 'JetBrains Mono', monospace;
                font-size: 13px;
                font-weight: 600;
                transition: all 0.2s ease;
                display: inline-flex;
                align-items: center;
                justify-content: center;
            }}
            .toolbar-btn:hover {{
                background: var(--bg-surface-high);
                color: var(--accent);
            }}
            .toolbar-divider {{
                width: 1px;
                height: 18px;
                background: var(--border);
                margin: 0 8px;
            }}
            .richtext-editor {{
                outline: none;
                font-size: 17px;
                line-height: 1.8;
                color: var(--text-primary);
                padding: 16px;
                background: transparent;
                font-family: Inter, -apple-system, sans-serif;
                cursor: text;
                min-height: 450px;
            }}
            .richtext-editor h1 {{
                font-size: 32px;
                font-weight: 800;
                margin-top: 24px;
                margin-bottom: 12px;
                letter-spacing: -0.03em;
            }}
            .richtext-editor h2 {{
                font-size: 24px;
                font-weight: 700;
                margin-top: 20px;
                margin-bottom: 8px;
                letter-spacing: -0.02em;
            }}
            .richtext-editor h3 {{
                font-size: 20px;
                font-weight: 600;
                margin-top: 16px;
                margin-bottom: 6px;
            }}
            .richtext-editor p {{
                margin-top: 8px;
                margin-bottom: 8px;
            }}
            .richtext-editor ul {{
                list-style-type: disc;
                padding-left: 24px;
                margin-top: 8px;
                margin-bottom: 8px;
            }}
            .richtext-editor ol {{
                list-style-type: decimal;
                padding-left: 24px;
                margin-top: 8px;
                margin-bottom: 8px;
            }}
            .richtext-editor li {{
                margin-bottom: 4px;
            }}
            .richtext-editor ul.task-list {{
                list-style-type: none !important;
                padding-left: 0 !important;
            }}
            .richtext-editor input[type="checkbox"] {{
                appearance: none;
                -webkit-appearance: none;
                width: 16px;
                height: 16px;
                border: 1px solid var(--border, #3b494b);
                border-radius: 4px;
                outline: none;
                background: transparent;
                cursor: pointer;
                display: inline-flex;
                align-items: center;
                justify-content: center;
                position: relative;
                transition: all 0.2s ease;
                margin-right: 8px;
                flex-shrink: 0;
            }}
            .richtext-editor input[type="checkbox"]:checked {{
                background: var(--accent, #00dbe9);
                border-color: var(--accent, #00dbe9);
            }}
            .richtext-editor input[type="checkbox"]:checked::after {{
                content: "✔";
                font-size: 10px;
                color: var(--bg-primary, #131313);
                font-weight: bold;
            }}
            .richtext-editor blockquote {{
                border-left: 4px solid var(--accent, #00dbe9);
                padding-left: 16px;
                color: var(--text-secondary, #b9cacb);
                font-style: italic;
                margin-top: 16px;
                margin-bottom: 16px;
            }}
            .richtext-editor pre {{
                position: relative;
                background: #0a0a0a;
                border: 1px solid var(--border, #3b494b);
                border-radius: 6px;
                padding: 24px 16px 16px;
                font-family: 'JetBrains Mono', monospace;
                font-size: 14px;
                line-height: 1.6;
                overflow-x: auto;
                margin-top: 16px;
                margin-bottom: 16px;
                color: var(--text-primary, #e5e2e1);
            }}
            .richtext-editor pre::before {{
                content: "code";
                position: absolute;
                top: 4px;
                right: 8px;
                font-size: 10px;
                font-family: 'JetBrains Mono', monospace;
                color: var(--text-muted, #849495);
                text-transform: uppercase;
                letter-spacing: 0.05em;
            }}
            .richtext-editor pre:has(code.language-typescript)::before {{ content: "typescript"; }}
            .richtext-editor pre:has(code.language-rust)::before {{ content: "rust"; }}
            .richtext-editor pre:has(code.language-javascript)::before {{ content: "javascript"; }}
            .richtext-editor pre:has(code.language-html)::before {{ content: "html"; }}
            .richtext-editor pre:has(code.language-css)::before {{ content: "css"; }}
            .richtext-editor pre:has(code.language-json)::before {{ content: "json"; }}
            
            .richtext-editor pre .code-copy-btn {{
                position: absolute;
                top: 6px;
                right: 90px;
                background: rgba(255, 255, 255, 0.05);
                border: 1px solid var(--border, #3b494b);
                border-radius: 4px;
                color: var(--text-muted, #849495);
                font-family: Inter, sans-serif;
                font-size: 11px;
                padding: 2px 8px;
                cursor: pointer;
                opacity: 0;
                transition: all 0.2s ease;
                z-index: 10;
                user-select: none;
                line-height: 1.2;
            }}
            .richtext-editor pre:hover .code-copy-btn {{
                opacity: 1;
            }}
            .richtext-editor pre .code-copy-btn:hover {{
                background: var(--bg-surface-high, #2a2a2a);
                color: var(--accent, #00dbe9);
                border-color: var(--accent, #00dbe9);
            }}
            
            .richtext-editor table {{
                width: 100%;
                border-collapse: collapse;
                margin: 20px 0;
                font-size: 14px;
            }}
            .richtext-editor th {{
                background: var(--bg-surface-high, #2a2a2a);
                color: var(--text-primary, #e5e2e1);
                font-weight: 600;
                border: 1px solid var(--border, #3b494b);
                padding: 8px 12px;
                text-align: left;
            }}
            .richtext-editor td {{
                border: 1px solid var(--border, #3b494b);
                padding: 8px 12px;
                color: var(--text-secondary, #b9cacb);
            }}
            .richtext-editor tr:nth-child(even) {{
                background: var(--bg-surface-low, #1c1b1b);
            }}
            .richtext-editor hr {{
                border: none;
                border-top: 1px solid var(--border, #3b494b);
                margin: 24px 0;
            }}
            .richtext-editor u {{
                text-decoration: underline;
            }}
            "#
        }
        div { style: "display: flex; flex-direction: column; height: 100%; width: 100%; position: relative; overflow-y: auto;",
            div {
                contenteditable: "true",
                id: "richtext-editor",
                class: "richtext-editor",
            }
        }
    }
}
