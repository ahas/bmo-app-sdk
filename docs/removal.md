# Removing an application

- **Every application generated from a manifest carries its removal** at
  `scripts/remove.sh`: the standing steps of taking one application off a machine, in the
  order that leaves nothing half-gone. The platform has no install verb's opposite; this is
  that opposite, held per application and read whole from its own manifest's values.
- **It plans by default:** it prints every step and changes nothing until run with `--yes`.
  The runtime data stays under the data root until run with `--purge-data` as well. The
  repository, its remote and its history stand untouched: removal is about the machine.
- **A step that cannot finish is said and passed over,** because a machine mid-removal is
  mended by running the removal again, and every step first reads whether it already stands
  done.

## The steps, in order

1. **The units go down:** each user unit named in the manifest's readings is disabled and
   stopped, then its file taken from the user's systemd folder, and the service manager is
   told.
2. **The registrations go:** the agency's repository row for the application is removed (an
   application with no row, or a row some job still holds a checkout of, is said, not
   forced), and the application's line is taken out of the machine's checkout register by
   the checkout path the manifest declares, or by its name where none is declared.
3. **The tool goes where the application registered one:** a hosted tool is taken out of the
   tool host's registry by that host's own verb before its placed folder is taken with the
   tree; an application with no tool has nothing here.
   Where the manifest names a `holder`, its set is withdrawn from the brain by the brain's own
   `holder withdraw`; a machine with no brain has nothing here.
4. **The deploy tree goes:** the releases, the build marks and the deploy link under the
   declared deploy root and nowhere else. The removal refuses a root that stands outside the
   home's share folder and outside `/srv/`, and refuses an empty one, so the paths it may
   take are exactly the paths a manifest may declare.
5. **The data stays or goes:** the data root is kept by default. With `--purge-data` an
   application whose `data_folder` is set loses that folder under the root and nothing else,
   and the removal refuses to take a whole data root that is the machine's shared one, so a
   console application can never stand as if it owned every service's data. An application
   with a data root of its own and no folder loses that root, under the same guard as the
   deploy tree.
6. **The machine's edges are the application's own document's:** the proxy host, DNS row and
   firewall rule that fronted it are removed by the steps its own deploy document states,
   since those belong to that application's machine facts, not to the base.

## Who runs it

- **The same hands that install:** a supervising agent on the owner's word for that
  application. A job that must show a removal working shows it planned: the script run bare
  prints the whole list, names the roots it would guard, and changes nothing.
