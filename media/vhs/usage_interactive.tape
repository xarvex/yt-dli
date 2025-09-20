Output media/usage_interactive.gif

Require ls
Require "yt-dli"
Require "yt-dlp"

Source media/vhs/init.tape

Sleep 500ms

Type "ls ~/.config/yt-dli"
Sleep 250ms
Enter Wait

Sleep 500ms

Type "yt-dli"
Sleep 500ms
Enter Wait /^?/

Sleep 1s

Down@500ms
Up@500ms
Sleep 500ms
Space
Sleep 1s
Down@500ms 3
Space@500ms 2
Up@500ms
Space
Sleep 1s
Enter Wait /^?/

Sleep 1s

Type@0 "https://www.youtube.com/watch?v=6IxqZOK_dpU"
Sleep 2s
Enter Wait

Sleep 2s
