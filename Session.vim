let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/dev/sex_offender/sex_offender_importer
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
set shortmess=aoO
argglobal
%argdel
$argadd src/bin/sximp.rs
edit src/bin/sximp.rs
let s:save_splitbelow = &splitbelow
let s:save_splitright = &splitright
set splitbelow splitright
wincmd _ | wincmd |
vsplit
1wincmd h
wincmd w
let &splitbelow = s:save_splitbelow
let &splitright = s:save_splitright
wincmd t
let s:save_winminheight = &winminheight
let s:save_winminwidth = &winminwidth
set winminheight=0
set winheight=1
set winminwidth=0
set winwidth=1
wincmd =
argglobal
balt src/config.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
silent! normal! zE
let &fdl = &fdl
let s:l = 333 - ((26 * winheight(0) + 19) / 39)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 333
normal! 051|
wincmd w
argglobal
if bufexists("term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh") | buffer term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh | else | edit term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh | endif
if &buftype ==# 'terminal'
  silent file term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh
endif
balt src/downloader/mod.rs
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 671 - ((19 * winheight(0) + 19) / 39)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 671
normal! 0
wincmd w
2wincmd w
wincmd =
tabnext 1
badd +337 src/bin/sximp.rs
badd +14 ~/dev/sex_offender/sex_offender_importer/Cargo.toml
badd +334 src/downloader/mod.rs
badd +368 src/importer/csv_importer.rs
badd +74 src/extractors/mod.rs
badd +52 src/config.rs
badd +23 src/util.rs
badd +66 src/importer/mod.rs
badd +1 src/importer/img.rs
badd +1 term
badd +9997 term://~/dev/sex_offender/sex_offender_importer//68684:/bin/zsh
badd +4 ~/dev/sex_offender/sex_offender_importer/src/lib.rs
badd +50 ~/dev/sex_offender/sex_offender_importer/src/downloader/records.rs
badd +15 src/downloader/archives.rs
badd +2313 sql/create_master.sql
badd +39 sql/nj_import.sql
badd +2357 sql/full_db.sql
badd +2222 sql/The_big_query.sql
badd +2492 term://~/dev/sex_offender/sex_offender_importer//93152:/bin/zsh
badd +0 term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20 shortmess=olxTOinFtfcI
let &winminheight = s:save_winminheight
let &winminwidth = s:save_winminwidth
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
let g:this_session = v:this_session
let g:this_obsession = v:this_session
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
