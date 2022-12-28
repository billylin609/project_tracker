Note:
 
There is a soln for error prompt press enter to continue in vim

the soln is to add ! after the w operation

:w!

plus when entering the mode plz use sudo before hand



it is interesting trying to use vundle for vim 

and it's proven to be successful but i remove a plugin in it

in linux the vimrc file is in etc

therefore the process is
cd ../../..

cd etc

cd vim 

vim vimrc

to access the file and follow the instruction give by vundle

Note: that the vundle is just a plugin management

if want to add a new Plugin must enter the vimrc file again and type

Plugin 'plugin_name'

and first time run :PluginInstall inside vim
or vim +PluginInstall
to initize the plugin

in addition to that if want to update the plugin we must run command
:PluginUpdate inside vim
or vim +PluginUpdate in terminal

the first plugin i choose is NERDTree

in order to install it i add
Plugin 'scrooloose/nerdtree'
Note: if note add this line in vimrc next time opening the terminal the plugin will disappear
and if we want to test whether the plugin exist or not can use
:PluginSearch

to install 
this is a directory managment tool

if want to use it
:NERDTree to call the plugin out

the second plugin install is
Plugin 'markdown'

currently don't know how to use it
