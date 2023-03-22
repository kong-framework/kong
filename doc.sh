#!/bin/bash
pandoc -i README.md -o website/index.html --metadata title="kong" -s --css=style.css
pandoc -i API.md -o website/api.html -s --css=style.css --toc
cargo doc --no-deps --target-dir website/rust/
