# Compatibility

## Mac

```
     Running `target/release/chip8rs 'roms/IBM Logo.ch8'`
Cli { program: "roms/IBM Logo.ch8", fps: 60, mul: 20, scale: 10, color: 4280427042, background: 4284903270, pitch: 220, quirk_vf_reset: false, quirk_memory: false, quirk_display_wait: false, quirk_clipping: false, quirk_shifting: false, quirk_jumping: false }
RendererInfo { name: "metal", flags: 14, texture_formats: [ARGB8888, ABGR8888, YV12, IYUV, Unknown, Unknown], max_texture_width: 16384, max_texture_height: 16384 }, default_pixel_format: ARGB8888, scale: (10.0, 10.0), logical_size: (64, 32), output_size: (640, 320), render_target_supported: true
coreaudio AudioSubsystem { _subsystem_drop: SubsystemDrop { _sdldrop: SdlDrop, flag: 16 } }

sdl2-config --version
2.28.3

Running `target/release/chip8rs 'roms/IBM Logo.ch8'`
Cli { program: "roms/IBM Logo.ch8", fps: 60, mul: 20, scale: 10, color: 4280427042, background: 4284903270, pitch: 220, quirk_vf_reset: false, quirk_memory: false, quirk_display_wait: false, quirk_clipping: false, quirk_shifting: false, quirk_jumping: false }
RendererInfo { name: "metal", flags: 14, texture_formats: [ARGB8888, ABGR8888, YV12, IYUV, Unknown, Unknown], max_texture_width: 16384, max_texture_height: 16384 }, default_pixel_format: ARGB8888, scale: (10.0, 10.0), logical_size: (64, 32), output_size: (640, 320), render_target_supported: true
coreaudio AudioSubsystem { _subsystem_drop: SubsystemDrop { _sdldrop: SdlDrop, flag: 16 } }

sdl2-config --version
2.30.9

sdl2 v0.35.2


Running `target/release/chip8rs 'roms/IBM Logo.ch8'`
Cli { program: "roms/IBM Logo.ch8", fps: 60, mul: 20, scale: 10, color: 4280427042, background: 4284903270, pitch: 220, quirk_vf_reset: false, quirk_memory: false, quirk_display_wait: false, quirk_clipping: false, quirk_shifting: false, quirk_jumping: false }
RendererInfo { name: "metal", flags: 14, texture_formats: [ARGB8888, ABGR8888, YV12, IYUV, NV12, NV21], max_texture_width: 16384, max_texture_height: 16384 }, default_pixel_format: ARGB8888, scale: (10.0, 10.0), logical_size: (64, 32), output_size: (640, 320), render_target_supported: true
coreaudio AudioSubsystem { _subsystem_drop: SubsystemDrop { _sdldrop: SdlDrop { marker: PhantomData<*mut ()> }, counter: 2, flag: 16 } }

sdl2-config --version
2.30.9

sdl2 v0.37.0

All works
```
