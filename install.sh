#!/bin/bash

function install_bird () {
   USR_DIR=$(pwd)
   TMP_DIR="/tmp/bird"

   ## RETRIEVE URL TO PROGRAM TARBALL
   echo -e "\nRetrieving download URL..."
   TAR_URL=$(curl -s https://api.github.com/repos/KevinSilvester/bird/releases/latest | grep "browser_download_url.*x86_64-unknown-linux-musl.tar.gz" | cut -d : -f 2,3 | tr -d \")
   TAR_FILE=$(echo $TAR_URL | awk -F/ '{print $NF}')

   if [ -z $TAR_URL ]; then
      echo -e "\nFailed retrieving download URL! ☹️"
      return 1
   fi

   echo -e "URL retrieved!\n"


   ## DOWNLAOD THE TARBALL
   echo -e "\nDownloading file"
   [ -d $TMP_DIR ] && rm -rf $TMP_DIR
   mkdir -p $TMP_DIR && cd $TMP_DIR

   wget $TAR_URL
   tar -xf $TAR_FILE

   if [ ! -f "bird" ]; then
      echo -e "\nFailed extracting binary! ☹️"
      return 1
   fi


   ## INSTALL PROGRAM
   echo -e "\nInstalling bird"
   [ ! -d "$HOME/.local/bin" ] && mkdir - "$HOME/.local/bin"
   [ -f "$HOME/.loca/bin/bird" ] && rm "$HOME/.loca/bin/bird"
   mv bird ~/.local/bin


   ## FINISH INSTALL
   if [[ ! ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
     echo -e "\nAdd '~/.local/bin' to you PATH to use bird"
   fi

   cd $USR_DIR
}

install_bird \
   && echo -e "\nbird install complete! 😄" \
   || echo -e "\nbird install failed! ☹️"
