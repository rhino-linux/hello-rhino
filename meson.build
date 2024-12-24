project(
  'hello-rhino',
  'rust',
  version: '0.1.0',
  license: 'GPL-3.0',
  meson_version: '>= 1.0',
)

cargo = find_program('cargo', required: true, version: '>=1.80')
find_program('rustc', required: true, version: '>=1.80')

i18n = import('i18n')
po_dir = '/usr/share/locale/'
subdir('po')

buildtype = get_option('buildtype')

rust_args = ['--locked']
if buildtype == 'release'
  rust_args += ['--release']
endif

build_file = meson.global_source_root() / 'target' / (buildtype == 'release' ? 'release' : 'debug') / meson.project_name()

custom_target(
  'hello-rhino',
  command: [
    cargo,
    'build',
    '-p', meson.project_name(),
    rust_args,
    '&&',
    'cp',
    '-p', build_file,
    '@OUTPUT@',
  ],
  output: 'hello-rhino',
  build_by_default: true,
  install: true,
  install_dir: get_option('bindir'),
)

install_data(
  files('hello-rhino.desktop'),
  install_dir: join_paths(get_option('datadir'), 'applications'),
)

install_data(
  files('org.rhinolinux.HelloRhino.svg'),
  install_dir: join_paths(get_option('datadir'), 'icons', 'hicolor', 'scalable', 'apps'),
)

install_data(
  files('LICENSE'),
  install_dir: join_paths(get_option('datadir'), 'licenses', meson.project_name()),
)
