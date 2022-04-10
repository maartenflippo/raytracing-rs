@echo off

cargo build --release
.\target\release\raytracing-rs.exe > image.ppm
..\ppm-viewer\target\release\rust-ui.exe .\image.ppm
