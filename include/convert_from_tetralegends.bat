@echo off
if not "%1"=="" (
  if exist "%1\*" (
    goto process
  )
)

echo Error: Argument is not directory
goto exit

:process
cd %~dp0

del *.ogg

set file[0]=ready
set file[1]=go
set file[2]=move
set file[3]=rotate
set file[4]=prespin
set file[5]=harddrop
set file[6]=locknohd
set file[7]=hold
set file[8]=gameover
set file[9]=erase1
set file[10]=erase4
set file[11]=erasenot4
set file[12]=tspin
set file[13]=tspin3
set file[14]=start
set file[15]=ko
set file[16]=levelup
set file[17]=levelupminor
set file[18]=bravo
set file[19]=lock
set "x=0"

:process_loop
setlocal EnableDelayedExpansion

if defined file[%x%] (
  call set "name=%%file[%x%]%%.ogg"

  if exist "%1\!name!" (
    echo Copying !name!...
    copy "%1\!name!" "!name!" > nul
  )

  set /a "x+=1"
  goto process_loop
)

:rename
if exist ko.ogg move /y ko.ogg game_over.ogg > nul
if exist gameover.ogg move /y gameover.ogg game_over.ogg > nul
if exist prespin.ogg move /y prespin.ogg rotate_spin.ogg > nul
if exist erase4.ogg move /y erase4.ogg erase_quad.ogg > nul
if exist erasenot4.ogg move /y erasenot4.ogg erase.ogg > nul
if exist erase1.ogg move /y erase1.ogg erase.ogg > nul
if exist harddrop.ogg move /y harddrop.ogg hard_drop.ogg > nul
if exist locknohd.ogg move /y locknohd.ogg lock.ogg > nul
if exist tspin3.ogg move /y tspin3.ogg tspin.ogg > nul
if exist start.ogg move /y start.ogg go.ogg > nul
if exist bravo.ogg move /y bravo.ogg level_up.ogg > nul
if exist levelupminor.ogg move /y levelupminor.ogg level_up.ogg > nul
if exist levelup.ogg move /y levelup.ogg level_up.ogg > nul

:make_dummy
if not exist hard_drop.ogg copy lock.ogg hard_drop.ogg > nul
if not exist move.ogg copy ogg.dummy move.ogg > nul
if not exist rotate.ogg copy ogg.dummy rotate.ogg > nul
if not exist rotate_spin.ogg copy ogg.dummy rotate_spin.ogg > nul
if not exist hold.ogg copy ogg.dummy hold.ogg > nul
if not exist ready.ogg copy ogg.dummy ready.ogg > nul
if not exist go.ogg copy ogg.dummy go.ogg > nul
if not exist tspin.ogg copy ogg.dummy tspin.ogg > nul

:exit
pause > nul
