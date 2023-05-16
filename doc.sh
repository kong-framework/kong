#!/bin/bash
pandoc -i README.md -o website/index.html --metadata title="kong" -s --css=style.css -H website/header.html
pandoc -i API.md -o website/api.html -s --css=style.css --toc -H website/header.html
cargo doc --no-deps --all-features --target-dir website/rust/
