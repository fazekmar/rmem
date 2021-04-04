# rmem

A CLI utility to summarize memory usage.
Highly inspired by [ps_mem](https://github.com/pixelb/ps_mem).

## WIP

On my system already much more accurate in comparison with ps_mem, but that doesn't necessarily mean it’s true for your system as well.

### Exmaple

```
$ rmem -S
  Private +    Shared =     Total       Swap    Program

 144.0 KB +  148.0 KB =  292.0 KB   252.0 KB    bash
   2.0 MB +  343.0 KB =    2.4 MB   548.0 KB    swaybar
   4.0 KB +    7.1 MB =    7.1 MB   872.0 KB    swaybg
  11.6 MB +   58.9 MB =   70.5 MB    50.4 MB    sway
 484.8 MB +   71.0 MB =  555.8 MB        0 B    codium (11)
 727.1 MB +   15.5 MB =  742.6 MB   636.0 KB    mpv (2)
   6.2 GB +  233.6 MB =    6.5 GB     1.3 GB    firefox-bin (12)
--------------------------------------------
                           7.9 GB     1.3 GB
============================================
```

## Install

```
$ cargo rmem
```

## Build

```
$ git clone https://github.com/fazekmar/rmem.git
$ cd rmem
$ cargo build --release
```
