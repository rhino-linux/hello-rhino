project(
  'hello-rhino',
  'rust',
  version: '0.1.0',
  license: 'GPL-3.0',
  meson_version: '>= 1.0',
)

cargo = find_program('cargo', required: true, version: '>=1.80')
find_program('rustc', required: true, version: '>=1.80')

bindir = '/usr/bin'

custom_target(
  'hello-rhino',
  command: [
    cargo,
    'build',
    '-p', meson.project_name(),
    '--locked',
    '--release', '&&',
    'cp',
    meson.global_source_root() / 'target' / 'release' / meson.project_name(),
    '@OUTPUT@',
  ],
  output: 'hello-rhino',
  build_by_default: true,
  install: true,
  install_dir: bindir,
)
