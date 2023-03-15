#!/bin/bash
pandoc -i doc/README.md -o doc/pub/index.html -s --css=style.css
pandoc -i doc/api.md -o doc/pub/api.html -s --css=style.css --toc
pandoc -i doc/kdata.md -o doc/pub/kdata.html -s --css=style.css --toc
pandoc -i doc/kollection.md -o doc/pub/kollection.html -s --css=style.css --toc
pandoc -i doc/krypto.md -o doc/pub/krypto.html -s --css=style.css --toc
