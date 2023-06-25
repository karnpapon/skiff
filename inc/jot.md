inspired by the a simplicity of [TextEditor](https://support.apple.com/en-ae/guide/textedit/welcome/mac), and an intuitive distractionless of [Left](https://github.com/hundredrabbits/Left). Powered by [Tauri](https://tauri.app), a framework for building tiny, blazingly fast binaries for all major desktop platforms.

<video alt="jot-demo" width="100%" autoplay loop controls>
  <source src="/media/images/jot/jot-demo.mp4" type="video/mp4">
</video>

# features
- lightweight (~3mb)
- writing-focus only
- quick inserts
- quick word referencing
- support extensions `.txt`, `.md`, `.doc`, `.docx`, `.rft`, `.rtf`

# download
[ download ](https://github.com/karnpapon/jot/releases) latest installer at release page, support major platforms(Win/OSX/Linux)

# usages
<details><summary>marker will be created by <code>#</code>, <code>##</code> or <code>###</code>. eg. <code>`# header marker`</code>, or <code>## sub-header marker</code> or <code>### marker</code> for quickly navigate between header.</summary><img alt="00" src="/media/images/jot/jot-header-tut.gif"></details>
<details><summary>for word referencing(eg. <code>word¹</code>), use <code>^</code> follow by any number eg. <code>word^1</code> and type <code>Cmd+Shift+6</code> (make sure the cursor is within the target word) will be converted to <code>word¹</code> and append reference to the end of the file.</summary><img alt="00" src="/media/images/jot/jot-ref-tut.gif"></details>

## shortcuts
- `Cmd+o` : open file
- `Cmd+n` : new file
- `Cmd+s` : save file
- `Cmd+Shift+s` : save as file
- `Cmd+f` : find text
- `Cmd+'` : toggle navigator
- `Cmd+b` : open url
- `Cmd+[` : move to previous marker
- `Cmd+]` : move to next marker
- `Cmd+Shift+6` : convert to footnote superscript
- `Cmd+Shift+;` : switch between dark/light theme

## inserts

- `Cmd+d` : Date
- `Cmd+t` : Time
- `Cmd+p` : Path
- `Cmd+h` : Header¹
- `Cmd+H` : Sub-Header¹
- `Cmd+/` : Comment

¹will create marker at navigator

## QA

<details><summary>is this WYSIWYG editor?</summary>it intentionally designed to focus on distractionless writing, thus <code>textarea</code> is being used. in this sense, <code>jot</code> is NOT a WYSIWYG editor.</details>
