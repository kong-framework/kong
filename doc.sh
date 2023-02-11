#!/bin/bash
pandoc -i doc/README.md -o doc/index.html -s --css=style.css --toc
pandoc -i doc/API.md -o doc/api.html -s --css=style.css --toc
pandoc -i doc/DATA.md -o doc/data.html -s --css=style.css --toc
