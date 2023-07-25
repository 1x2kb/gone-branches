#!/bin/bash
if ${{ matrix.platform }} == x86_64-pc-windows-gnu; 
    then export artifact_extension=.exe
fi