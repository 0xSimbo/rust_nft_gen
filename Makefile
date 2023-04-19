-include .env

# Path: Makefile

.PHONY :; help;

help :; @echo "Please use \`make <target>' where <target> is one of" @echo "  build_images to build the Docker images" @echo "  upload_metadata to upload the metadata to PyPI" @echo "  upload_images to upload the images to Docker Hub" @echo "  upload_all to upload all to PyPI and Docker Hub"


generate_images :; cargo run --release
upload_metadata :; cargo run --release && @echo "Uploading metadata to PyPI" twine upload dist/* -r pypi
upload_images :; @echo "Uploading images to Docker Hub" docker push"
upload_all :; @echo "Uploading all to PyPI and Docker Hub"

randomize :; py randomize.py