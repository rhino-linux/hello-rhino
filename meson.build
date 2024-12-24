project(
  'hello-rhino',
  'rust',
  version: '0.1.0',
  license: 'GPL-3.0',
  meson_version: '>= 1.0',
)

cargo = find_program('cargo', required: true, version: '>=1.80')
find_program('rustc', required: true, version: '>=1.80')

buildtype = get_option('buildtype')

rust_args = ['--locked']
if buildtype == 'release'
  rust_args += ['--release']
endif

custom_target(
  'hello-rhino-build',
  command: [cargo] + rust_args,
  build_by_default: true,
  output: [
    meson.global_source_root() / 'target' / (buildtype == 'release' ? 'release' : 'debug') / meson.project_name(),
  ],
)

desktop_file = files('hello-rhino.desktop')
install_data(desktop_file, install_dir: join_paths(get_option('datadir'), 'applications'))

icon_file = files('org.rhinolinux.HelloRhino.svg')
install_data(
  icon_file,
  install_dir: join_paths(get_option('datadir'), 'icons', 'hicolor', 'scalable', 'apps'),
)

license_file = files('LICENSE')
install_data(
  license_file,
  install_dir: join_paths(get_option('datadir'), 'licenses', meson.project_name()),
)
