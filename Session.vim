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
balt src/importer/img.rs
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
let s:l = 1 - ((0 * winheight(0) + 19) / 39)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 1
normal! 010|
wincmd w
argglobal
if bufexists("term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh") | buffer term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh | else | edit term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh | endif
if &buftype ==# 'terminal'
  silent file term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh
endif
setlocal fdm=manual
setlocal fde=0
setlocal fmr={{{,}}}
setlocal fdi=#
setlocal fdl=0
setlocal fml=1
setlocal fdn=20
setlocal fen
let s:l = 5525 - ((38 * winheight(0) + 19) / 39)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 5525
normal! 02|
wincmd w
2wincmd w
wincmd =
tabnext 1
badd +99 src/bin/sximp.rs
badd +14 ~/dev/sex_offender/sex_offender_importer/Cargo.toml
badd +194 src/importer/csv_importer.rs
badd +12 src/extractors/mod.rs
badd +33 src/config.rs
badd +22 src/util.rs
badd +104 src/importer/mod.rs
badd +33 src/importer/img.rs
badd +1 term
badd +9997 term://~/dev/sex_offender/sex_offender_importer//68684:/bin/zsh
badd +6 ~/dev/sex_offender/sex_offender_importer/src/lib.rs
badd +2313 sql/create_master.sql
badd +39 sql/nj_import.sql
badd +2357 sql/full_db.sql
badd +2222 sql/The_big_query.sql
badd +2492 term://~/dev/sex_offender/sex_offender_importer//93152:/bin/zsh
badd +1204 term://~/dev/sex_offender/sex_offender_importer//22712:/bin/zsh
badd +394 term://~/dev/sex_offender/sex_offender_importer//7873:/bin/zsh
badd +43 ~/.cargo/registry/src/github.com-1ecc6299db9ec823/ftp-3.0.1/src/ftp.rs
badd +1 ~/dev/sex_offender/sex_offender_importer/tests/seximporter_tests.rs
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
