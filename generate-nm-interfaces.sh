#!/usr/bin/env bash
set -e

readonly NETWORKMANAGER_RELEASE="1.24.2"

if !which git &> /dev/null; then
  echo "Git must be installed!!"
  exit 1
fi

root="$(readlink -f "$(dirname "$0")")"
if [[ ! -d "$root" ]]; then
  echo "Could not find root $root"
  exit 1
fi

mkdir -p $root/tmp
git clone https://gitlab.freedesktop.org/NetworkManager/NetworkManager.git $root/tmp/
cd tmp
git checkout tags/$NETWORKMANAGER_RELEASE
cd ..

if ! hash dbus-codegen-rust 2> /dev/null; then
  echo "Could not find dbus-codegen-rust binary. Do you want to install it using Cargo?"
  echo -n "[Yn] > "
  read -r c
  if [[ $c == "y" || $c == "Y" ]]; then
    cargo install dbus-codegen
  else
    exit 1
  fi
fi

dest="$root/src/api/gen"
mkdir -p $dest

alltraits=()
allmods=()
for spec in $root/tmp/introspection/*.xml; do
  basename=$(basename "$spec" .xml)
  trait=( $(IFS=. ; printf '%s ' $basename) )
  trait=$(echo ${trait[@]^} | tr -d "[:space:]")
  alltraits+=($trait)
  modname=${basename#org.freedesktop.NetworkManager}
  modname=nm${modname//./_}
  modname=$(echo $modname | tr '[:upper:]' '[:lower:]')
  allmods+=($modname)
  dest_file="${dest}/${modname}".rs
  echo "Generating code from $(basename "${spec}") to ${modname}…"
  dbus-codegen-rust -m None < "$spec" > "$dest_file"
done

echo "#![allow(warnings)]" > $dest/mod.rs

for mod in ${allmods[@]}; do
  echo "mod "$mod";" >> $dest/mod.rs
done

echo -e "\n" >> $dest/mod.rs

for ((i=0;i<${#allmods[@]};++i)); do
    echo "pub(super) use ${allmods[i]}::${alltraits[i]};" >> $dest/mod.rs
done

echo "Formatting code... "
rustfmt $dest/*.rs
echo "Done."

echo "Cleaning up..."
rm -rf $root/tmp
echo "Done."