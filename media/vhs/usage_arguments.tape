Output media/usage_arguments.gif

Require "yt-dli"
Require "yt-dlp"

Source media/vhs/init.tape

Sleep 500ms

Type@0 `eval "$(yt-dli completions bash)"`
Sleep 1s
Enter Wait

Sleep 1s

Type "yt-dli --profile"
Sleep 500ms
Backspace 8
Type "p "
Tab@250ms 2
Sleep 500ms
Type "7"
Sleep 250ms
Tab
Sleep 1s
Backspace
Type ","
Tab@250ms 2
Type "sh"
Sleep 250ms
Tab

Sleep 1s

Type@0 "https://www.youtube.com/watch?v=6IxqZOK_dpU"
Sleep 2s
Enter Wait

Sleep 2s
