#!/usr/bin/env bash

{ # This ensures the entire script is downloaded.
	wget https://raw.githubusercontent.com/TiagoCavalcante/rust-template/main/scripts/rust-template.tar.gz
	tar -xvf rust-template.tar
	chmod +x rust-template
	sudo cp rust-template /usr/local/bin/rust-template
	echo "rust-template is now installed! 🚀"
}
