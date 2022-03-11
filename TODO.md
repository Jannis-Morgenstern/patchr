# TODO

- [x] read cli arguments
  - 1st: name of the package to be patched
  - package-manager: `npm | yarn | pnpm`
  - patch-dir: path
  - exclude: Regex relative to root of package to be pathed
  - include: Regex relative to root of package to be pathed
  - create-issue: open browser with issue draft if package specified a repo
- [ ] determine package-manager (cli argument > [package].lock)
- [ ] create a temporary git repo
- [ ] install package with package-manager that should be patched
- [ ] copy dirty package into the temporary git repo
- [ ] diff package